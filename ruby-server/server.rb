require "json"
require "sinatra"
require "sinatra/cors"
require "verus-gem"

set :allow_origin, "*"
set :allow_methods, "POST"

before do
  content_type :json
end

post "/validate-email" do
  Verus.validate_email(JSON.parse(request.body.read)).to_json
end

post "/validate-date" do
  Verus.validate_date(JSON.parse(request.body.read)).to_json
end
