require 'json'

def allowed_attributes
  [:first_name, :last_name, :username, :email]
end

def worker(req)
  sampleJson = <<-JSON
    {
      "first_name": "Tracie",
      "last_name": "Schroeder",
      "username": "tracie.schroeder",
      "email": "tracie.schroeder@email.com",
      "password": "secret"
    }
  JSON

  user = JSON.parse sampleJson, symbolize_names: true

  user_response = Hash.new
  allowed_attributes.each do |attribute|
    user_response[attribute] = user[attribute]
  end

  res = Response.new({
    "user": user_response,
    "some_file_contents": File.read("/tmp/file.txt")
  }.to_json)
  res.headers["x-generated-by"] = "wasm-workers-server"
  res
end
