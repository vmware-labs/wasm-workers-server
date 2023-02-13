require 'json';

def json_to_request(input)
  json = JSON.parse(input);

  Request.new(
    json['url'],
    json['body'],
    json['method'],
    json['headers'],
    json['params']
  )
end

class Request
  attr_reader :url, :body, :method, :headers, :params

  def initialize(url, body, method, headers, params)
    @url = url
    @body = body
    @method = method
    @headers = headers
    @params = params
  end
end

class Response
  attr_accessor :body, :status_code, :headers

  def initialize(body, status_code = 200, headers = {})
    @body = body
    @status_code = status_code
    @headers = headers
  end

  def self.ok(body)
    new(body)
  end

  def self.created(body)
    new(body, 201)
  end

  def self.not_found(body)
    new(body, 404)
  end

  def print_headers
    serialized = "{"

    headers.each do |k, v|
      serialized = "#{serialized} \"#{k}\": \"#{v}\""
    end

    "#{serialized} }"
  end

  def to_s
    JSON.generate({
      data: body,
      status: status_code,
      headers: headers,
      kv: {}
    })
  end
end
