require 'json';

def json_to_request(input)
  json = JSON.parse(input);

  Request.new(
    json['url'],
    json['body'],
    json['method'],
    json['headers'],
    json['params'],
    json['kv']
  )
end

class Cache
  @@kv = {}

  def self.init(kv)
    @@kv = kv
  end

  def self.dump
    @@kv
  end

  def self.get(key)
    @@kv[key.to_s]
  end

  def self.set(key, value)
    @@kv[key.to_s] = value.to_s
  end
end

class Request
  attr_reader :url, :body, :method, :headers, :params

  def initialize(url, body, method, headers, params, kv)
    @url = url
    @body = body
    @method = method
    @headers = headers
    @params = params

    # Initializes the cache
    Cache.init(kv)
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

  def to_s
    content = body
    encoded = false

    if body.encoding.name != "UTF-8"
      content = [body].pack("m0")
      encoded = true
    end

    JSON.generate({
      data: content,
      status: status_code,
      headers: headers,
      kv: Cache.dump,
      base64: encoded
    })
  end
end
