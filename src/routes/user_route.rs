use crate::models::user_model::print_user_model;
pub fn print_user_route() {
  // crate::routes::health_route::print_health_route();
  super::health_route::print_health_route(); 
  print_user_model();
  println!("user_route");
}