//! An example of decoding requests from an HTML form element

extern crate futures;
extern crate gotham;
extern crate hyper;
extern crate mime;
extern crate url;

use futures::{future, Future, Stream};
use hyper::{Body, StatusCode};
use url::form_urlencoded;

use gotham::handler::{HandlerFuture, IntoHandlerError};
use gotham::helpers::http::response::create_response;
use gotham::router::builder::{build_simple_router, DefineSingleRoute, DrawRoutes};
use gotham::router::Router;
use gotham::state::{FromState, State};

// use tract_core::ndarray;
use tract_core::prelude::*;

// fn init_plan() -> SimplePlan<TensorInfo, Borrow<ModelImpl<TI, O>>> {
//   // load the model
//   let mut model =
//       tract_tensorflow::tensorflow().model_for_path("mobilenet_v2_1.4_224_frozen.pb")?;

//   // specify input type and shape
//   model.set_input_fact(0, TensorFact::dt_shape(f32::datum_type(), tvec!(1, 224, 224, 3)))?;

//   // optimize the model and get an execution plan
//   let model = model.into_optimized()?;
//   let plan = SimplePlan::new(&model)?;
//   plan;
// }

// fn prediction_handler(state: State) -> (State, &'static str) {
//   // open image, resize it and make a Tensor out of it
//   let image = image::open("grace_hopper.jpg").unwrap().to_rgb();
//   let resized = image::imageops::resize(&image, 224, 224, ::image::FilterType::Triangle);
//   let image: Tensor = ndarray::Array4::from_shape_fn((1, 224, 224, 3), |(_, y, x, c)| {
//       resized[(x as _, y as _)][c] as f32 / 255.0
//   })
//   .into();

//   // run the plan on the input
//   let result = plan.run(tvec!(image))?;

//   // find and display the max value with its index
//   let best = result[0]
//       .to_array_view::<f32>()?
//       .iter()
//       .cloned()
//       .zip(1..)
//       .max_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
//   (state, best)
// }

/// Extracts the elements of the POST request and responds with the form keys and values
fn form_handler(mut state: State) -> Box<HandlerFuture> {
  let f = Body::take_from(&mut state)
    .concat2()
    .then(|full_body| match full_body {
      Ok(valid_body) => {
        let body_content = valid_body.into_bytes();
        // Perform decoding on request body
        let form_data = form_urlencoded::parse(&body_content).into_owned();
        // Add form keys and values to response body
        let mut res_body = String::new();
        for (key, value) in form_data {
          let res_body_line = format!("{}: {}\n", key, value);
          res_body.push_str(&res_body_line);
        }
        let res = create_response(&state, StatusCode::OK, mime::TEXT_PLAIN, res_body);
        future::ok((state, res))
      }
      Err(e) => future::err((state, e.into_handler_error())),
    });

  Box::new(f)
}

/// Create a `Router`
fn router(plan: SimplePlan) -> Router {
  build_simple_router(|route| {
    // route.get("/").to(prediction_handler);
    route.post("/").to(form_handler);
  })
}

/// Start a server and use a `Router` to dispatch requests
pub fn main() {
  // load the model
  let mut model =
      tract_tensorflow::tensorflow().model_for_path("mobilenet_v2_1.4_224_frozen.pb").unwrap();

  // specify input type and shape
  model.set_input_fact(0, TensorFact::dt_shape(f32::datum_type(), tvec!(1, 224, 224, 3))).unwrap();

  // optimize the model and get an execution plan
  let model = model.into_optimized().unwrap();
  let plan = SimplePlan::new(&model).unwrap();

  let addr = "127.0.0.1:7878";
  println!("Listening for requests at http://{}", addr);
  gotham::start(addr, router(plan))
}
