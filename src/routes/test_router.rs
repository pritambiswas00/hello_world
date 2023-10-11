use axum::Json;
use serde::{ Deserialize, Serialize };



//Body of a post request
#[derive(Debug, Deserialize, Serialize)]
 pub struct  BodyIncoming {
      message:String,
      status: u8

}



pub async fn test_handler(Json(body): Json<BodyIncoming>)->Json<BodyIncoming>{
       println!("{:?} body of the incoming post", body);
       Json(body)
}