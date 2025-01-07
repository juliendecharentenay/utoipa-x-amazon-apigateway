
#[utoipa::path(
  get,
  path = "/",
  tags = [ "pets" ],
  description = "PetStore HTML web page containing API usage information",
  responses(
    (
      status = 200,
      description = "Successful operation",
      headers(
        ("Content-Type" = String, description = "Media type of request")
      )
    )
  )
)]
fn get_root() {}

#[derive(utoipa::ToSchema)]
struct Pet {
  id: u32,
  #[schema(rename = "type")]
  ty: String,
  price: f32,
}

#[utoipa::path(
  get,
  path = "/pets",
  tags = [ "pets" ],
  description = "List all pets",
  params(
    ( "type" = Option<String>, Query, description = "The type of pet to retrieve", ),
    ( "page" = Option<String>, Query, description = "Page number of results to return.", ),
  ),
  responses(
    ( status = 200,
      description = "Successful operation",
      headers(
        ( "Access-Control-Allow-Origin" = String, description = "URI that may access the resource" )
      ),
      body = Vec<Pet>,
    ),
  ),
)]
fn get_pets() {}

utoipa_x_amazon_apigateway::define_operation_modifier!(
  GetPetsXAmazonApigatewayIntegration,
  "/pets",
  utoipa::openapi::HttpMethod::Get,
  utoipa_x_amazon_apigateway::x_amazon_apigateway::integration::http_passthrough_uri_method_parameters_responses(
    Some(utoipa_x_amazon_apigateway::enums::PassthroughBehavior::WhenNoMatch),
    "http://petstore.execute-api.eu-west-1.amazonaws.com/petstore/pets",
    utoipa::openapi::HttpMethod::Get,
    Some(vec![
      ("integration.request.querystring.page", "method.request.querystring.page"),
      ("integration.request.querystring.type", "method.request.querystring.type")
    ]),
    Some(vec![
      ( "default",
        utoipa_x_amazon_apigateway::x_amazon_apigateway::integration::Response::from_status_code_parameters(
          "200",
          vec![
            ("Access-Control-Allow-Origin", "'*'"),
          ]
        ).unwrap()
      )
    ]),
  )
);


#[derive(utoipa::OpenApi)]
#[openapi(info(
  title = "PetStore",
  description = "Your first API with Amazon API Gateway. This is a sample API that integrates via HTTP with our demo Pet Store endpoints"),
  paths(get_root, get_pets),
  modifiers(
    &GetRootXAmazonApigatewayIntegration,
    &GetPetsXAmazonApigatewayIntegration,
  )
)]
struct ApiDoc;

utoipa_x_amazon_apigateway::define_operation_modifier!(
  GetRootXAmazonApigatewayIntegration,
  "/",
  utoipa::openapi::HttpMethod::Get,
  utoipa_x_amazon_apigateway::x_amazon_apigateway::integration::mock_passthrough_request_templates_responses(
    Some(utoipa_x_amazon_apigateway::enums::PassthroughBehavior::WhenNoMatch),
    Some(vec![
      ("application/json", "{\"statusCode\": 200}"),
    ]),
    Some(vec![
      ( "default", 
        utoipa_x_amazon_apigateway::x_amazon_apigateway::integration::Response::from_status_code_templates_parameters(
          "200",
          vec![
            ("text/html", "<html>\n    <head>\n        <style>\n        body {\n            color: #333;\n            font-family: Sans-serif;\n            max-width: 800px;\n            margin: auto;\n        }\n        </style>\n    </head>\n    <body>\n        <h1>Welcome to your Pet Store API</h1>\n        <p>\n            You have successfully deployed your first API. You are seeing this HTML page because the <code>GET</code> method to the root resource of your API returns this content as a Mock integration.\n        </p>\n        <p>\n            The Pet Store API contains the <code>/pets</code> and <code>/pets/{petId}</code> resources. By making a <a href=\"/$context.stage/pets/\" target=\"_blank\"><code>GET</code> request</a> to <code>/pets</code> you can retrieve a list of Pets in your API. If you are looking for a specific pet, for example the pet with ID 1, you can make a <a href=\"/$context.stage/pets/1\" target=\"_blank\"><code>GET</code> request</a> to <code>/pets/1</code>.\n        </p>\n        <p>\n            You can use a REST client such as <a href=\"https://www.getpostman.com/\" target=\"_blank\">Postman</a> to test the <code>POST</code> methods in your API to create a new pet. Use the sample body below to send the <code>POST</code> request:\n        </p>\n        <pre>\n{\n    \"type\" : \"cat\",\n    \"price\" : 123.11\n}\n        </pre>\n    </body>\n</html>")
          ],
          vec![
            ("Content-Type", "'text/html'"),
          ],
        ).unwrap()
      )
    ]),
  )
);

fn main() {
  use utoipa::OpenApi;
  println!("{}", ApiDoc::openapi().to_pretty_json().unwrap());
}
