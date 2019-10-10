use futures::{future, Future, Stream};
use gotham::handler::{HandlerFuture, IntoHandlerError};
use gotham::helpers::http::response::create_response;
use gotham::router::builder::{build_simple_router, DefineSingleRoute, DrawRoutes};
use gotham::router::Router;
use gotham::state::{FromState, State};
use hyper::{Body, StatusCode};
use image;
use mime;
use regex::bytes::Regex;
use tract_core::{ndarray, prelude::*};

/// Extracts the image from a POST request and responds with a prediction tuple (probability, class)
fn prediction_handler(mut state: State) -> Box<HandlerFuture> {
    let f = Body::take_from(&mut state)
        .concat2()
        .then(|full_body| match full_body {
            Ok(valid_body) => {
                // load the model
                let mut model = tract_tensorflow::tensorflow()
                    .model_for_path("mobilenet_v2_1.4_224_frozen.pb")
                    .unwrap();

                // specify input type and shape
                model
                    .set_input_fact(
                        0,
                        TensorFact::dt_shape(f32::datum_type(), tvec!(1, 224, 224, 3)),
                    )
                    .unwrap();

                // optimize the model and get an execution plan
                let model = model.into_optimized().unwrap();
                let plan = SimplePlan::new(&model).unwrap();

                // extract the image from the body as input
                let body_content = valid_body.into_bytes();
                let re = Regex::new(r"\r\n\r\n").unwrap();
                let contents: Vec<_> = re.split(body_content.as_ref()).collect();
                let image = image::load_from_memory(contents[1]).unwrap().to_rgb();
                let resized =
                    image::imageops::resize(&image, 224, 224, ::image::FilterType::Triangle);
                let image: Tensor =
                    ndarray::Array4::from_shape_fn((1, 224, 224, 3), |(_, y, x, c)| {
                        resized[(x as _, y as _)][c] as f32 / 255.0
                    })
                    .into();

                // run the plan on the input
                let result = plan.run(tvec!(image)).unwrap();

                // find and display the max value with its index
                let best = result[0]
                    .to_array_view::<f32>()
                    .unwrap()
                    .iter()
                    .cloned()
                    .zip(1..)
                    .max_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

                // respond with the prediction tuple
                let res = create_response(
                    &state,
                    StatusCode::OK,
                    mime::TEXT_PLAIN,
                    format!("{:?}", best.unwrap()),
                );
                future::ok((state, res))
            }
            Err(e) => future::err((state, e.into_handler_error())),
        });

    Box::new(f)
}

/// Create a `Router`
fn router() -> Router {
    build_simple_router(|route| {
        route.post("/").to(prediction_handler);
    })
}

/// Start a server and use a `Router` to dispatch requests
pub fn main() {
    let addr = "127.0.0.1:7878";
    println!("Listening for requests at http://{}", addr);
    gotham::start(addr, router())
}
