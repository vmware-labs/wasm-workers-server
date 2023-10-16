(() => {
  // node_modules/hono/dist/types.js
  var FetchEventLike = class {
  };
  var FetchEvent = class extends FetchEventLike {
  };

  // node_modules/hono/dist/utils/url.js
  var splitPath = (path) => {
    const paths = path.split("/");
    if (paths[0] === "") {
      paths.shift();
    }
    return paths;
  };
  var splitRoutingPath = (path) => {
    const groups = [];
    for (let i = 0; ;) {
      let replaced = false;
      path = path.replace(/\{[^}]+\}/g, (m) => {
        const mark = `@\\${i}`;
        groups[i] = [mark, m];
        i++;
        replaced = true;
        return mark;
      });
      if (!replaced) {
        break;
      }
    }
    const paths = path.split("/");
    if (paths[0] === "") {
      paths.shift();
    }
    for (let i = groups.length - 1; i >= 0; i--) {
      const [mark] = groups[i];
      for (let j = paths.length - 1; j >= 0; j--) {
        if (paths[j].indexOf(mark) !== -1) {
          paths[j] = paths[j].replace(mark, groups[i][1]);
          break;
        }
      }
    }
    return paths;
  };
  var patternCache = {};
  var getPattern = (label) => {
    if (label === "*") {
      return "*";
    }
    const match = label.match(/^\:([^\{\}]+)(?:\{(.+)\})?$/);
    if (match) {
      if (!patternCache[label]) {
        if (match[2]) {
          patternCache[label] = [label, match[1], new RegExp("^" + match[2] + "$")];
        } else {
          patternCache[label] = [label, match[1], true];
        }
      }
      return patternCache[label];
    }
    return null;
  };
  var getPath = (request) => {
    const match = request.url.match(/^https?:\/\/[^/]+(\/[^?]*)/);
    return match ? match[1] : "";
  };
  var getQueryStrings = (url) => {
    const queryIndex = url.indexOf("?", 8);
    return queryIndex === -1 ? "" : "?" + url.slice(queryIndex + 1);
  };
  var getPathNoStrict = (request) => {
    const result = getPath(request);
    return result.length > 1 && result[result.length - 1] === "/" ? result.slice(0, -1) : result;
  };
  var mergePath = (...paths) => {
    let p = "";
    let endsWithSlash = false;
    for (let path of paths) {
      if (p[p.length - 1] === "/") {
        p = p.slice(0, -1);
        endsWithSlash = true;
      }
      if (path[0] !== "/") {
        path = `/${path}`;
      }
      if (path === "/" && endsWithSlash) {
        p = `${p}/`;
      } else if (path !== "/") {
        p = `${p}${path}`;
      }
      if (path === "/" && p === "") {
        p = "/";
      }
    }
    return p;
  };
  var checkOptionalParameter = (path) => {
    const match = path.match(/^(.+|)(\/\:[^\/]+)\?$/);
    if (!match)
      return null;
    const base = match[1];
    const optional = base + match[2];
    return [base === "" ? "/" : base.replace(/\/$/, ""), optional];
  };
  var _decodeURI = (value) => {
    if (!/[%+]/.test(value)) {
      return value;
    }
    if (value.indexOf("+") !== -1) {
      value = value.replace(/\+/g, " ");
    }
    return /%/.test(value) ? decodeURIComponent_(value) : value;
  };
  var _getQueryParam = (url, key, multiple) => {
    let encoded;
    if (!multiple && key && !/[%+]/.test(key)) {
      let keyIndex2 = url.indexOf(`?${key}`, 8);
      if (keyIndex2 === -1) {
        keyIndex2 = url.indexOf(`&${key}`, 8);
      }
      while (keyIndex2 !== -1) {
        const trailingKeyCode = url.charCodeAt(keyIndex2 + key.length + 1);
        if (trailingKeyCode === 61) {
          const valueIndex = keyIndex2 + key.length + 2;
          const endIndex = url.indexOf("&", valueIndex);
          return _decodeURI(url.slice(valueIndex, endIndex === -1 ? void 0 : endIndex));
        } else if (trailingKeyCode == 38 || isNaN(trailingKeyCode)) {
          return "";
        }
        keyIndex2 = url.indexOf(`&${key}`, keyIndex2 + 1);
      }
      encoded = /[%+]/.test(url);
      if (!encoded) {
        return void 0;
      }
    }
    const results = {};
    encoded ?? (encoded = /[%+]/.test(url));
    let keyIndex = url.indexOf("?", 8);
    while (keyIndex !== -1) {
      const nextKeyIndex = url.indexOf("&", keyIndex + 1);
      let valueIndex = url.indexOf("=", keyIndex);
      if (valueIndex > nextKeyIndex && nextKeyIndex !== -1) {
        valueIndex = -1;
      }
      let name = url.slice(
        keyIndex + 1,
        valueIndex === -1 ? nextKeyIndex === -1 ? void 0 : nextKeyIndex : valueIndex
      );
      if (encoded) {
        name = _decodeURI(name);
      }
      keyIndex = nextKeyIndex;
      if (name === "") {
        continue;
      }
      let value;
      if (valueIndex === -1) {
        value = "";
      } else {
        value = url.slice(valueIndex + 1, nextKeyIndex === -1 ? void 0 : nextKeyIndex);
        if (encoded) {
          value = _decodeURI(value);
        }
      }
      if (multiple) {
        ;
        (results[name] ?? (results[name] = [])).push(value);
      } else {
        results[name] ?? (results[name] = value);
      }
    }
    return key ? results[key] : results;
  };
  var getQueryParam = _getQueryParam;
  var getQueryParams = (url, key) => {
    return _getQueryParam(url, key, true);
  };
  var decodeURIComponent_ = decodeURIComponent;

  // node_modules/hono/dist/utils/cookie.js
  var validCookieNameRegEx = /^[\w!#$%&'*.^`|~+-]+$/;
  var validCookieValueRegEx = /^[ !#-:<-[\]-~]*$/;
  var parse = (cookie, name) => {
    const pairs = cookie.trim().split(";");
    return pairs.reduce((parsedCookie, pairStr) => {
      pairStr = pairStr.trim();
      const valueStartPos = pairStr.indexOf("=");
      if (valueStartPos === -1)
        return parsedCookie;
      const cookieName = pairStr.substring(0, valueStartPos).trim();
      if (name && name !== cookieName || !validCookieNameRegEx.test(cookieName))
        return parsedCookie;
      let cookieValue = pairStr.substring(valueStartPos + 1).trim();
      if (cookieValue.startsWith('"') && cookieValue.endsWith('"'))
        cookieValue = cookieValue.slice(1, -1);
      if (validCookieValueRegEx.test(cookieValue))
        parsedCookie[cookieName] = decodeURIComponent_(cookieValue);
      return parsedCookie;
    }, {});
  };
  var _serialize = (name, value, opt = {}) => {
    let cookie = `${name}=${value}`;
    if (opt && typeof opt.maxAge === "number" && opt.maxAge >= 0) {
      cookie += `; Max-Age=${Math.floor(opt.maxAge)}`;
    }
    if (opt.domain) {
      cookie += `; Domain=${opt.domain}`;
    }
    if (opt.path) {
      cookie += `; Path=${opt.path}`;
    }
    if (opt.expires) {
      cookie += `; Expires=${opt.expires.toUTCString()}`;
    }
    if (opt.httpOnly) {
      cookie += "; HttpOnly";
    }
    if (opt.secure) {
      cookie += "; Secure";
    }
    if (opt.sameSite) {
      cookie += `; SameSite=${opt.sameSite}`;
    }
    if (opt.partitioned) {
      cookie += "; Partitioned";
    }
    return cookie;
  };
  var serialize = (name, value, opt = {}) => {
    value = encodeURIComponent(value);
    return _serialize(name, value, opt);
  };

  // node_modules/hono/dist/utils/stream.js
  var StreamingApi = class {
    constructor(writable) {
      this.writable = writable;
      this.writer = writable.getWriter();
      this.encoder = new TextEncoder();
    }
    async write(input) {
      try {
        if (typeof input === "string") {
          input = this.encoder.encode(input);
        }
        await this.writer.write(input);
      } catch (e) {
      }
      return this;
    }
    async writeln(input) {
      await this.write(input + "\n");
      return this;
    }
    sleep(ms) {
      return new Promise((res) => setTimeout(res, ms));
    }
    async close() {
      try {
        await this.writer.close();
      } catch (e) {
      }
    }
    async pipe(body) {
      this.writer.releaseLock();
      await body.pipeTo(this.writable, { preventClose: true });
      this.writer = this.writable.getWriter();
    }
  };

  // node_modules/hono/dist/context.js
  var TEXT_PLAIN = "text/plain; charset=UTF-8";
  var Context = class {
    constructor(req, options) {
      this.env = {};
      this._var = {};
      this.finalized = false;
      this.error = void 0;
      this._status = 200;
      this._h = void 0;
      this._pH = void 0;
      this._init = true;
      this._renderer = (content) => this.html(content);
      this.notFoundHandler = () => new Response();
      this.render = (...args) => this._renderer(...args);
      this.setRenderer = (renderer) => {
        this._renderer = renderer;
      };
      this.header = (name, value, options2) => {
        if (value === void 0) {
          if (this._h) {
            this._h.delete(name);
          } else if (this._pH) {
            delete this._pH[name.toLocaleLowerCase()];
          }
          if (this.finalized) {
            this.res.headers.delete(name);
          }
          return;
        }
        if (options2?.append) {
          if (!this._h) {
            this._init = false;
            this._h = new Headers(this._pH);
            this._pH = {};
          }
          this._h.append(name, value);
        } else {
          if (this._h) {
            this._h.set(name, value);
          } else {
            this._pH ?? (this._pH = {});
            this._pH[name.toLowerCase()] = value;
          }
        }
        if (this.finalized) {
          if (options2?.append) {
            this.res.headers.append(name, value);
          } else {
            this.res.headers.set(name, value);
          }
        }
      };
      this.status = (status) => {
        this._status = status;
      };
      this.set = (key, value) => {
        this._var ?? (this._var = {});
        this._var[key] = value;
      };
      this.get = (key) => {
        return this._var ? this._var[key] : void 0;
      };
      this.newResponse = (data, arg, headers) => {
        if (this._init && !headers && !arg && this._status === 200) {
          return new Response(data, {
            headers: this._pH
          });
        }
        if (arg && typeof arg !== "number") {
          const res = new Response(data, arg);
          const contentType = this._pH?.["content-type"];
          if (contentType) {
            res.headers.set("content-type", contentType);
          }
          return res;
        }
        const status = arg ?? this._status;
        this._pH ?? (this._pH = {});
        this._h ?? (this._h = new Headers());
        for (const [k, v] of Object.entries(this._pH)) {
          this._h.set(k, v);
        }
        if (this._res) {
          this._res.headers.forEach((v, k) => {
            this._h?.set(k, v);
          });
          for (const [k, v] of Object.entries(this._pH)) {
            this._h.set(k, v);
          }
        }
        headers ?? (headers = {});
        for (const [k, v] of Object.entries(headers)) {
          if (typeof v === "string") {
            this._h.set(k, v);
          } else {
            this._h.delete(k);
            for (const v2 of v) {
              this._h.append(k, v2);
            }
          }
        }
        return new Response(data, {
          status,
          headers: this._h
        });
      };
      this.body = (data, arg, headers) => {
        return typeof arg === "number" ? this.newResponse(data, arg, headers) : this.newResponse(data, arg);
      };
      this.text = (text, arg, headers) => {
        if (!this._pH) {
          if (this._init && !headers && !arg) {
            return new Response(text);
          }
          this._pH = {};
        }
        if (this._pH["content-type"]) {
          this._pH["content-type"] = TEXT_PLAIN;
        }
        return typeof arg === "number" ? this.newResponse(text, arg, headers) : this.newResponse(text, arg);
      };
      this.json = (object, arg, headers) => {
        const body = JSON.stringify(object);
        this._pH ?? (this._pH = {});
        this._pH["content-type"] = "application/json; charset=UTF-8";
        return typeof arg === "number" ? this.newResponse(body, arg, headers) : this.newResponse(body, arg);
      };
      this.jsonT = (object, arg, headers) => {
        const response = typeof arg === "number" ? this.json(object, arg, headers) : this.json(object, arg);
        return {
          response,
          data: object,
          format: "json",
          status: response.status
        };
      };
      this.html = (html, arg, headers) => {
        this._pH ?? (this._pH = {});
        this._pH["content-type"] = "text/html; charset=UTF-8";
        return typeof arg === "number" ? this.newResponse(html, arg, headers) : this.newResponse(html, arg);
      };
      this.redirect = (location, status = 302) => {
        this._h ?? (this._h = new Headers());
        this._h.set("Location", location);
        return this.newResponse(null, status);
      };
      this.streamText = (cb, arg, headers) => {
        headers ?? (headers = {});
        this.header("content-type", TEXT_PLAIN);
        this.header("x-content-type-options", "nosniff");
        this.header("transfer-encoding", "chunked");
        return this.stream(cb, arg, headers);
      };
      this.stream = (cb, arg, headers) => {
        const { readable, writable } = new TransformStream();
        const stream = new StreamingApi(writable);
        cb(stream).finally(() => stream.close());
        return typeof arg === "number" ? this.newResponse(readable, arg, headers) : this.newResponse(readable, arg);
      };
      this.cookie = (name, value, opt) => {
        const cookie = serialize(name, value, opt);
        this.header("set-cookie", cookie, { append: true });
      };
      this.notFound = () => {
        return this.notFoundHandler(this);
      };
      this.req = req;
      if (options) {
        this._exCtx = options.executionCtx;
        this.env = options.env;
        if (options.notFoundHandler) {
          this.notFoundHandler = options.notFoundHandler;
        }
      }
    }
    get event() {
      if (this._exCtx instanceof FetchEvent) {
        return this._exCtx;
      } else {
        throw Error("This context has no FetchEvent");
      }
    }
    get executionCtx() {
      if (this._exCtx) {
        return this._exCtx;
      } else {
        throw Error("This context has no ExecutionContext");
      }
    }
    get res() {
      this._init = false;
      return this._res || (this._res = new Response("404 Not Found", { status: 404 }));
    }
    set res(_res) {
      this._init = false;
      if (this._res && _res) {
        this._res.headers.delete("content-type");
        this._res.headers.forEach((v, k) => {
          _res.headers.set(k, v);
        });
      }
      this._res = _res;
      this.finalized = true;
    }
    get var() {
      return { ...this._var };
    }
    get runtime() {
      const global = globalThis;
      if (global?.Deno !== void 0) {
        return "deno";
      }
      if (global?.Bun !== void 0) {
        return "bun";
      }
      if (typeof global?.WebSocketPair === "function") {
        return "workerd";
      }
      if (typeof global?.EdgeRuntime === "string") {
        return "edge-light";
      }
      if (global?.fastly !== void 0) {
        return "fastly";
      }
      if (global?.__lagon__ !== void 0) {
        return "lagon";
      }
      if (global?.process?.release?.name === "node") {
        return "node";
      }
      return "other";
    }
  };

  // node_modules/hono/dist/compose.js
  var compose = (middleware, onError, onNotFound) => {
    const middlewareLength = middleware.length;
    return (context, next) => {
      let index = -1;
      return dispatch(0);
      function dispatch(i) {
        if (i <= index) {
          throw new Error("next() called multiple times");
        }
        let handler = middleware[i];
        index = i;
        if (i === middlewareLength && next)
          handler = next;
        let res;
        let isError = false;
        if (!handler) {
          if (context instanceof Context && context.finalized === false && onNotFound) {
            res = onNotFound(context);
          }
        } else {
          try {
            res = handler(context, () => {
              const dispatchRes = dispatch(i + 1);
              return dispatchRes instanceof Promise ? dispatchRes : Promise.resolve(dispatchRes);
            });
          } catch (err) {
            if (err instanceof Error && context instanceof Context && onError) {
              context.error = err;
              res = onError(err, context);
              isError = true;
            } else {
              throw err;
            }
          }
        }
        if (!(res instanceof Promise)) {
          if (res !== void 0 && "response" in res) {
            res = res["response"];
          }
          if (res && (context.finalized === false || isError)) {
            context.res = res;
          }
          return context;
        } else {
          return res.then((res2) => {
            if (res2 !== void 0 && "response" in res2) {
              res2 = res2["response"];
            }
            if (res2 && context.finalized === false) {
              context.res = res2;
            }
            return context;
          }).catch(async (err) => {
            if (err instanceof Error && context instanceof Context && onError) {
              context.error = err;
              context.res = await onError(err, context);
              return context;
            }
            throw err;
          });
        }
      }
    };
  };

  // node_modules/hono/dist/http-exception.js
  var HTTPException = class extends Error {
    constructor(status = 500, options) {
      super(options?.message);
      this.res = options?.res;
      this.status = status;
    }
    getResponse() {
      if (this.res) {
        return this.res;
      }
      return new Response(this.message, {
        status: this.status
      });
    }
  };

  // node_modules/hono/dist/utils/body.js
  var parseBody = async (request) => {
    let body = {};
    const contentType = request.headers.get("Content-Type");
    if (contentType && (contentType.startsWith("multipart/form-data") || contentType.startsWith("application/x-www-form-urlencoded"))) {
      const formData = await request.formData();
      if (formData) {
        const form = {};
        formData.forEach((value, key) => {
          if (key.slice(-2) === "[]") {
            if (!form[key]) {
              form[key] = [value.toString()];
            } else {
              if (Array.isArray(form[key])) {
                ;
                form[key].push(value.toString());
              }
            }
          } else {
            form[key] = value;
          }
        });
        body = form;
      }
    }
    return body;
  };

  // node_modules/hono/dist/request.js
  var HonoRequest = class {
    constructor(request, path = "/", paramData) {
      this.bodyCache = {};
      this.cachedBody = (key) => {
        const { bodyCache, raw } = this;
        const cachedBody = bodyCache[key];
        if (cachedBody)
          return cachedBody;
        if (bodyCache.arrayBuffer) {
          return (async () => {
            return await new Response(bodyCache.arrayBuffer)[key]();
          })();
        }
        return bodyCache[key] = raw[key]();
      };
      this.raw = request;
      this.path = path;
      this.paramData = paramData;
      this.vData = {};
    }
    param(key) {
      if (this.paramData) {
        if (key) {
          const param = this.paramData[key];
          return param ? /\%/.test(param) ? decodeURIComponent_(param) : param : void 0;
        } else {
          const decoded = {};
          for (const [key2, value] of Object.entries(this.paramData)) {
            if (value && typeof value === "string") {
              decoded[key2] = /\%/.test(value) ? decodeURIComponent_(value) : value;
            }
          }
          return decoded;
        }
      }
      return null;
    }
    query(key) {
      return getQueryParam(this.url, key);
    }
    queries(key) {
      return getQueryParams(this.url, key);
    }
    header(name) {
      if (name)
        return this.raw.headers.get(name.toLowerCase()) ?? void 0;
      const headerData = {};
      this.raw.headers.forEach((value, key) => {
        headerData[key] = value;
      });
      return headerData;
    }
    cookie(key) {
      const cookie = this.raw.headers.get("Cookie");
      if (!cookie)
        return;
      const obj = parse(cookie);
      if (key) {
        const value = obj[key];
        return value;
      } else {
        return obj;
      }
    }
    async parseBody() {
      if (this.bodyCache.parsedBody)
        return this.bodyCache.parsedBody;
      const parsedBody = await parseBody(this);
      this.bodyCache.parsedBody = parsedBody;
      return parsedBody;
    }
    json() {
      return this.cachedBody("json");
    }
    text() {
      return this.cachedBody("text");
    }
    arrayBuffer() {
      return this.cachedBody("arrayBuffer");
    }
    blob() {
      return this.cachedBody("blob");
    }
    formData() {
      return this.cachedBody("formData");
    }
    addValidatedData(target, data) {
      this.vData[target] = data;
    }
    valid(target) {
      return this.vData[target];
    }
    get url() {
      return this.raw.url;
    }
    get method() {
      return this.raw.method;
    }
    get headers() {
      return this.raw.headers;
    }
    get body() {
      return this.raw.body;
    }
    get bodyUsed() {
      return this.raw.bodyUsed;
    }
    get integrity() {
      return this.raw.integrity;
    }
    get keepalive() {
      return this.raw.keepalive;
    }
    get referrer() {
      return this.raw.referrer;
    }
    get signal() {
      return this.raw.signal;
    }
  };

  // node_modules/hono/dist/router.js
  var METHOD_NAME_ALL = "ALL";
  var METHOD_NAME_ALL_LOWERCASE = "all";
  var METHODS = ["get", "post", "put", "delete", "options", "patch"];
  var UnsupportedPathError = class extends Error {
  };

  // node_modules/hono/dist/hono-base.js
  function defineDynamicClass() {
    return class {
    };
  }
  var notFoundHandler = (c) => {
    return c.text("404 Not Found", 404);
  };
  var errorHandler = (err, c) => {
    if (err instanceof HTTPException) {
      return err.getResponse();
    }
    console.trace(err);
    const message = "Internal Server Error";
    return c.text(message, 500);
  };
  var Hono = class extends defineDynamicClass() {
    constructor(init = {}) {
      super();
      this._basePath = "/";
      this.path = "/";
      this.routes = [];
      this.notFoundHandler = notFoundHandler;
      this.errorHandler = errorHandler;
      this.head = () => {
        console.warn("`app.head()` is no longer used. `app.get()` implicitly handles the HEAD method.");
        return this;
      };
      this.handleEvent = (event) => {
        return this.dispatch(event.request, event, void 0, event.request.method);
      };
      this.fetch = (request, Env, executionCtx) => {
        return this.dispatch(request, executionCtx, Env, request.method);
      };
      this.request = (input, requestInit, Env, executionCtx) => {
        if (input instanceof Request) {
          if (requestInit !== void 0) {
            input = new Request(input, requestInit);
          }
          return this.fetch(input, Env, executionCtx);
        }
        input = input.toString();
        const path = /^https?:\/\//.test(input) ? input : `http://localhost${mergePath("/", input)}`;
        const req = new Request(path, requestInit);
        return this.fetch(req, Env, executionCtx);
      };
      this.fire = () => {
        addEventListener("fetch", (event) => {
          event.respondWith(this.dispatch(event.request, event, void 0, event.request.method));
        });
      };
      const allMethods = [...METHODS, METHOD_NAME_ALL_LOWERCASE];
      allMethods.map((method) => {
        this[method] = (args1, ...args) => {
          if (typeof args1 === "string") {
            this.path = args1;
          } else {
            this.addRoute(method, this.path, args1);
          }
          args.map((handler) => {
            if (typeof handler !== "string") {
              this.addRoute(method, this.path, handler);
            }
          });
          return this;
        };
      });
      this.on = (method, path, ...handlers) => {
        if (!method)
          return this;
        this.path = path;
        for (const m of [method].flat()) {
          handlers.map((handler) => {
            this.addRoute(m.toUpperCase(), this.path, handler);
          });
        }
        return this;
      };
      this.use = (arg1, ...handlers) => {
        if (typeof arg1 === "string") {
          this.path = arg1;
        } else {
          handlers.unshift(arg1);
        }
        handlers.map((handler) => {
          this.addRoute(METHOD_NAME_ALL, this.path, handler);
        });
        return this;
      };
      const strict = init.strict ?? true;
      delete init.strict;
      Object.assign(this, init);
      this.getPath = strict ? init.getPath ?? getPath : getPathNoStrict;
    }
    clone() {
      const clone = new Hono({
        router: this.router,
        getPath: this.getPath
      });
      clone.routes = this.routes;
      return clone;
    }
    route(path, app2) {
      const subApp = this.basePath(path);
      if (!app2) {
        return subApp;
      }
      app2.routes.map((r) => {
        const handler = app2.errorHandler === errorHandler ? r.handler : async (c, next) => (await compose([r.handler], app2.errorHandler)(c, next)).res;
        subApp.addRoute(r.method, r.path, handler);
      });
      return this;
    }
    basePath(path) {
      const subApp = this.clone();
      subApp._basePath = mergePath(this._basePath, path);
      return subApp;
    }
    onError(handler) {
      this.errorHandler = handler;
      return this;
    }
    notFound(handler) {
      this.notFoundHandler = handler;
      return this;
    }
    showRoutes() {
      const length = 8;
      this.routes.map((route) => {
        console.log(
          `\x1B[32m${route.method}\x1B[0m ${" ".repeat(length - route.method.length)} ${route.path}`
        );
      });
    }
    mount(path, applicationHandler, optionHandler) {
      const mergedPath = mergePath(this._basePath, path);
      const pathPrefixLength = mergedPath === "/" ? 0 : mergedPath.length;
      const handler = async (c, next) => {
        let executionContext = void 0;
        try {
          executionContext = c.executionCtx;
        } catch {
        }
        const options = optionHandler ? optionHandler(c) : [c.env, executionContext];
        const optionsArray = Array.isArray(options) ? options : [options];
        const queryStrings = getQueryStrings(c.req.url);
        const res = await applicationHandler(
          new Request(
            new URL((c.req.path.slice(pathPrefixLength) || "/") + queryStrings, c.req.url),
            c.req.raw
          ),
          ...optionsArray
        );
        if (res)
          return res;
        await next();
      };
      this.addRoute(METHOD_NAME_ALL, mergePath(path, "*"), handler);
      return this;
    }
    get routerName() {
      this.matchRoute("GET", "/");
      return this.router.name;
    }
    addRoute(method, path, handler) {
      method = method.toUpperCase();
      if (this._basePath) {
        path = mergePath(this._basePath, path);
      }
      this.router.add(method, path, handler);
      const r = { path, method, handler };
      this.routes.push(r);
    }
    matchRoute(method, path) {
      return this.router.match(method, path) || { handlers: [], params: {} };
    }
    handleError(err, c) {
      if (err instanceof Error) {
        return this.errorHandler(err, c);
      }
      throw err;
    }
    dispatch(request, executionCtx, env, method) {
      if (method === "HEAD") {
        return (async () => new Response(null, await this.dispatch(request, executionCtx, env, "GET")))();
      }
      const path = this.getPath(request, { env });
      const { handlers, params } = this.matchRoute(method, path);
      const c = new Context(new HonoRequest(request, path, params), {
        env,
        executionCtx,
        notFoundHandler: this.notFoundHandler
      });
      if (handlers.length === 1) {
        let res;
        try {
          res = handlers[0](c, async () => {
          });
          if (!res) {
            return this.notFoundHandler(c);
          }
        } catch (err) {
          return this.handleError(err, c);
        }
        if (res instanceof Response)
          return res;
        if ("response" in res) {
          res = res.response;
        }
        if (res instanceof Response)
          return res;
        return (async () => {
          let awaited;
          try {
            awaited = await res;
            if (awaited !== void 0 && "response" in awaited) {
              awaited = awaited["response"];
            }
            if (!awaited) {
              return this.notFoundHandler(c);
            }
          } catch (err) {
            return this.handleError(err, c);
          }
          return awaited;
        })();
      }
      const composed = compose(handlers, this.errorHandler, this.notFoundHandler);
      return (async () => {
        try {
          const tmp = composed(c);
          const context = tmp.constructor.name === "Promise" ? await tmp : tmp;
          if (!context.finalized) {
            throw new Error(
              "Context is not finalized. You may forget returning Response object or `await next()`"
            );
          }
          return context.res;
        } catch (err) {
          return this.handleError(err, c);
        }
      })();
    }
  };

  // node_modules/hono/dist/router/reg-exp-router/node.js
  var LABEL_REG_EXP_STR = "[^/]+";
  var ONLY_WILDCARD_REG_EXP_STR = ".*";
  var TAIL_WILDCARD_REG_EXP_STR = "(?:|/.*)";
  var PATH_ERROR = Symbol();
  function compareKey(a, b) {
    if (a.length === 1) {
      return b.length === 1 ? a < b ? -1 : 1 : -1;
    }
    if (b.length === 1) {
      return 1;
    }
    if (a === ONLY_WILDCARD_REG_EXP_STR || a === TAIL_WILDCARD_REG_EXP_STR) {
      return 1;
    } else if (b === ONLY_WILDCARD_REG_EXP_STR || b === TAIL_WILDCARD_REG_EXP_STR) {
      return -1;
    }
    if (a === LABEL_REG_EXP_STR) {
      return 1;
    } else if (b === LABEL_REG_EXP_STR) {
      return -1;
    }
    return a.length === b.length ? a < b ? -1 : 1 : b.length - a.length;
  }
  var Node = class {
    constructor() {
      this.children = {};
    }
    insert(tokens, index, paramMap, context, pathErrorCheckOnly) {
      if (tokens.length === 0) {
        if (this.index !== void 0) {
          throw PATH_ERROR;
        }
        if (pathErrorCheckOnly) {
          return;
        }
        this.index = index;
        return;
      }
      const [token, ...restTokens] = tokens;
      const pattern = token === "*" ? restTokens.length === 0 ? ["", "", ONLY_WILDCARD_REG_EXP_STR] : ["", "", LABEL_REG_EXP_STR] : token === "/*" ? ["", "", TAIL_WILDCARD_REG_EXP_STR] : token.match(/^\:([^\{\}]+)(?:\{(.+)\})?$/);
      let node;
      if (pattern) {
        const name = pattern[1];
        const regexpStr = pattern[2] || LABEL_REG_EXP_STR;
        node = this.children[regexpStr];
        if (!node) {
          if (Object.keys(this.children).some(
            (k) => k !== ONLY_WILDCARD_REG_EXP_STR && k !== TAIL_WILDCARD_REG_EXP_STR
          )) {
            throw PATH_ERROR;
          }
          if (pathErrorCheckOnly) {
            return;
          }
          node = this.children[regexpStr] = new Node();
          if (name !== "") {
            node.varIndex = context.varIndex++;
          }
        }
        if (!pathErrorCheckOnly && name !== "") {
          if (paramMap.some((p) => p[0] === name)) {
            throw new Error("Duplicate param name");
          }
          paramMap.push([name, node.varIndex]);
        }
      } else {
        node = this.children[token];
        if (!node) {
          if (Object.keys(this.children).some(
            (k) => k.length > 1 && k !== ONLY_WILDCARD_REG_EXP_STR && k !== TAIL_WILDCARD_REG_EXP_STR
          )) {
            throw PATH_ERROR;
          }
          if (pathErrorCheckOnly) {
            return;
          }
          node = this.children[token] = new Node();
        }
      }
      node.insert(restTokens, index, paramMap, context, pathErrorCheckOnly);
    }
    buildRegExpStr() {
      const childKeys = Object.keys(this.children).sort(compareKey);
      const strList = childKeys.map((k) => {
        const c = this.children[k];
        return (typeof c.varIndex === "number" ? `(${k})@${c.varIndex}` : k) + c.buildRegExpStr();
      });
      if (typeof this.index === "number") {
        strList.unshift(`#${this.index}`);
      }
      if (strList.length === 0) {
        return "";
      }
      if (strList.length === 1) {
        return strList[0];
      }
      return "(?:" + strList.join("|") + ")";
    }
  };

  // node_modules/hono/dist/router/reg-exp-router/trie.js
  var Trie = class {
    constructor() {
      this.context = { varIndex: 0 };
      this.root = new Node();
    }
    insert(path, index, pathErrorCheckOnly) {
      const paramMap = [];
      const groups = [];
      for (let i = 0; ;) {
        let replaced = false;
        path = path.replace(/\{[^}]+\}/g, (m) => {
          const mark = `@\\${i}`;
          groups[i] = [mark, m];
          i++;
          replaced = true;
          return mark;
        });
        if (!replaced) {
          break;
        }
      }
      const tokens = path.match(/(?::[^\/]+)|(?:\/\*$)|./g) || [];
      for (let i = groups.length - 1; i >= 0; i--) {
        const [mark] = groups[i];
        for (let j = tokens.length - 1; j >= 0; j--) {
          if (tokens[j].indexOf(mark) !== -1) {
            tokens[j] = tokens[j].replace(mark, groups[i][1]);
            break;
          }
        }
      }
      this.root.insert(tokens, index, paramMap, this.context, pathErrorCheckOnly);
      return paramMap;
    }
    buildRegExp() {
      let regexp = this.root.buildRegExpStr();
      if (regexp === "") {
        return [/^$/, [], []];
      }
      let captureIndex = 0;
      const indexReplacementMap = [];
      const paramReplacementMap = [];
      regexp = regexp.replace(/#(\d+)|@(\d+)|\.\*\$/g, (_, handlerIndex, paramIndex) => {
        if (typeof handlerIndex !== "undefined") {
          indexReplacementMap[++captureIndex] = Number(handlerIndex);
          return "$()";
        }
        if (typeof paramIndex !== "undefined") {
          paramReplacementMap[Number(paramIndex)] = ++captureIndex;
          return "";
        }
        return "";
      });
      return [new RegExp(`^${regexp}`), indexReplacementMap, paramReplacementMap];
    }
  };

  // node_modules/hono/dist/router/reg-exp-router/router.js
  var methodNames = [METHOD_NAME_ALL, ...METHODS].map((method) => method.toUpperCase());
  var emptyParam = {};
  var nullMatcher = [/^$/, [], {}];
  var wildcardRegExpCache = {};
  function buildWildcardRegExp(path) {
    return wildcardRegExpCache[path] ?? (wildcardRegExpCache[path] = new RegExp(
      path === "*" ? "" : `^${path.replace(/\/\*/, "(?:|/.*)")}$`
    ));
  }
  function clearWildcardRegExpCache() {
    wildcardRegExpCache = {};
  }
  function buildMatcherFromPreprocessedRoutes(routes) {
    const trie = new Trie();
    const handlerData = [];
    if (routes.length === 0) {
      return nullMatcher;
    }
    const routesWithStaticPathFlag = routes.map((route) => [!/\*|\/:/.test(route[0]), ...route]).sort(
      ([isStaticA, pathA], [isStaticB, pathB]) => isStaticA ? 1 : isStaticB ? -1 : pathA.length - pathB.length
    );
    const staticMap = {};
    for (let i = 0, j = -1, len = routesWithStaticPathFlag.length; i < len; i++) {
      const [pathErrorCheckOnly, path, handlers] = routesWithStaticPathFlag[i];
      if (pathErrorCheckOnly) {
        staticMap[path] = { handlers, params: emptyParam };
      } else {
        j++;
      }
      let paramMap;
      try {
        paramMap = trie.insert(path, j, pathErrorCheckOnly);
      } catch (e) {
        throw e === PATH_ERROR ? new UnsupportedPathError(path) : e;
      }
      if (pathErrorCheckOnly) {
        continue;
      }
      handlerData[j] = paramMap.length === 0 ? [{ handlers, params: emptyParam }, null] : [handlers, paramMap];
    }
    const [regexp, indexReplacementMap, paramReplacementMap] = trie.buildRegExp();
    for (let i = 0, len = handlerData.length; i < len; i++) {
      const paramMap = handlerData[i][1];
      if (paramMap) {
        for (let j = 0, len2 = paramMap.length; j < len2; j++) {
          paramMap[j][1] = paramReplacementMap[paramMap[j][1]];
        }
      }
    }
    const handlerMap = [];
    for (const i in indexReplacementMap) {
      handlerMap[i] = handlerData[indexReplacementMap[i]];
    }
    return [regexp, handlerMap, staticMap];
  }
  function findMiddleware(middleware, path) {
    if (!middleware) {
      return void 0;
    }
    for (const k of Object.keys(middleware).sort((a, b) => b.length - a.length)) {
      if (buildWildcardRegExp(k).test(path)) {
        return [...middleware[k]];
      }
    }
    return void 0;
  }
  var RegExpRouter = class {
    constructor() {
      this.name = "RegExpRouter";
      this.middleware = { [METHOD_NAME_ALL]: {} };
      this.routes = { [METHOD_NAME_ALL]: {} };
    }
    add(method, path, handler) {
      var _a;
      const { middleware, routes } = this;
      if (!middleware || !routes) {
        throw new Error("Can not add a route since the matcher is already built.");
      }
      if (methodNames.indexOf(method) === -1)
        methodNames.push(method);
      if (!middleware[method]) {
        ;
        [middleware, routes].forEach((handlerMap) => {
          handlerMap[method] = {};
          Object.keys(handlerMap[METHOD_NAME_ALL]).forEach((p) => {
            handlerMap[method][p] = [...handlerMap[METHOD_NAME_ALL][p]];
          });
        });
      }
      if (path === "/*") {
        path = "*";
      }
      if (/\*$/.test(path)) {
        const re = buildWildcardRegExp(path);
        if (method === METHOD_NAME_ALL) {
          Object.keys(middleware).forEach((m) => {
            var _a2;
            (_a2 = middleware[m])[path] || (_a2[path] = findMiddleware(middleware[m], path) || findMiddleware(middleware[METHOD_NAME_ALL], path) || []);
          });
        } else {
          (_a = middleware[method])[path] || (_a[path] = findMiddleware(middleware[method], path) || findMiddleware(middleware[METHOD_NAME_ALL], path) || []);
        }
        Object.keys(middleware).forEach((m) => {
          if (method === METHOD_NAME_ALL || method === m) {
            Object.keys(middleware[m]).forEach((p) => {
              re.test(p) && middleware[m][p].push(handler);
            });
          }
        });
        Object.keys(routes).forEach((m) => {
          if (method === METHOD_NAME_ALL || method === m) {
            Object.keys(routes[m]).forEach((p) => re.test(p) && routes[m][p].push(handler));
          }
        });
        return;
      }
      const paths = checkOptionalParameter(path) || [path];
      for (let i = 0, len = paths.length; i < len; i++) {
        const path2 = paths[i];
        Object.keys(routes).forEach((m) => {
          var _a2;
          if (method === METHOD_NAME_ALL || method === m) {
            (_a2 = routes[m])[path2] || (_a2[path2] = [
              ...findMiddleware(middleware[m], path2) || findMiddleware(middleware[METHOD_NAME_ALL], path2) || []
            ]);
            routes[m][path2].push(handler);
          }
        });
      }
    }
    match(method, path) {
      clearWildcardRegExpCache();
      const matchers = this.buildAllMatchers();
      this.match = (method2, path2) => {
        const matcher = matchers[method2];
        const staticMatch = matcher[2][path2];
        if (staticMatch) {
          return staticMatch;
        }
        const match = path2.match(matcher[0]);
        if (!match) {
          return null;
        }
        const index = match.indexOf("", 1);
        const [handlers, paramMap] = matcher[1][index];
        if (!paramMap) {
          return handlers;
        }
        const params = {};
        for (let i = 0, len = paramMap.length; i < len; i++) {
          params[paramMap[i][0]] = match[paramMap[i][1]];
        }
        return { handlers, params };
      };
      return this.match(method, path);
    }
    buildAllMatchers() {
      const matchers = {};
      methodNames.forEach((method) => {
        matchers[method] = this.buildMatcher(method) || matchers[METHOD_NAME_ALL];
      });
      this.middleware = this.routes = void 0;
      return matchers;
    }
    buildMatcher(method) {
      const routes = [];
      let hasOwnRoute = method === METHOD_NAME_ALL;
      [this.middleware, this.routes].forEach((r) => {
        const ownRoute = r[method] ? Object.keys(r[method]).map((path) => [path, r[method][path]]) : [];
        if (ownRoute.length !== 0) {
          hasOwnRoute || (hasOwnRoute = true);
          routes.push(...ownRoute);
        } else if (method !== METHOD_NAME_ALL) {
          routes.push(
            ...Object.keys(r[METHOD_NAME_ALL]).map((path) => [path, r[METHOD_NAME_ALL][path]])
          );
        }
      });
      if (!hasOwnRoute) {
        return null;
      } else {
        return buildMatcherFromPreprocessedRoutes(routes);
      }
    }
  };

  // node_modules/hono/dist/router/smart-router/router.js
  var SmartRouter = class {
    constructor(init) {
      this.name = "SmartRouter";
      this.routers = [];
      this.routes = [];
      Object.assign(this, init);
    }
    add(method, path, handler) {
      if (!this.routes) {
        throw new Error("Can not add a route since the matcher is already built.");
      }
      this.routes.push([method, path, handler]);
    }
    match(method, path) {
      if (!this.routes) {
        throw new Error("Fatal error");
      }
      const { routers, routes } = this;
      const len = routers.length;
      let i = 0;
      let res;
      for (; i < len; i++) {
        const router = routers[i];
        try {
          routes.forEach((args) => {
            router.add(...args);
          });
          res = router.match(method, path);
        } catch (e) {
          if (e instanceof UnsupportedPathError) {
            continue;
          }
          throw e;
        }
        this.match = router.match.bind(router);
        this.routers = [router];
        this.routes = void 0;
        break;
      }
      if (i === len) {
        throw new Error("Fatal error");
      }
      this.name = `SmartRouter + ${this.activeRouter.name}`;
      return res || null;
    }
    get activeRouter() {
      if (this.routes || this.routers.length !== 1) {
        throw new Error("No active router has been determined yet.");
      }
      return this.routers[0];
    }
  };

  // node_modules/hono/dist/router/trie-router/node.js
  function findParam(node, name) {
    for (let i = 0, len = node.patterns.length; i < len; i++) {
      if (typeof node.patterns[i] === "object" && node.patterns[i][1] === name) {
        return true;
      }
    }
    const nodes = Object.values(node.children);
    for (let i = 0, len = nodes.length; i < len; i++) {
      if (findParam(nodes[i], name)) {
        return true;
      }
    }
    return false;
  }
  var Node2 = class {
    constructor(method, handler, children) {
      this.order = 0;
      this.children = children || {};
      this.methods = [];
      this.name = "";
      if (method && handler) {
        const m = {};
        m[method] = { handler, score: 0, name: this.name };
        this.methods = [m];
      }
      this.patterns = [];
      this.handlerSetCache = {};
    }
    insert(method, path, handler) {
      this.name = `${method} ${path}`;
      this.order = ++this.order;
      let curNode = this;
      const parts = splitRoutingPath(path);
      const parentPatterns = [];
      const errorMessage = (name) => {
        return `Duplicate param name, use another name instead of '${name}' - ${method} ${path} <--- '${name}'`;
      };
      for (let i = 0, len = parts.length; i < len; i++) {
        const p = parts[i];
        if (Object.keys(curNode.children).includes(p)) {
          parentPatterns.push(...curNode.patterns);
          curNode = curNode.children[p];
          continue;
        }
        curNode.children[p] = new Node2();
        const pattern = getPattern(p);
        if (pattern) {
          if (typeof pattern === "object") {
            for (let j = 0, len2 = parentPatterns.length; j < len2; j++) {
              if (typeof parentPatterns[j] === "object" && parentPatterns[j][1] === pattern[1]) {
                throw new Error(errorMessage(pattern[1]));
              }
            }
            if (Object.values(curNode.children).some((n) => findParam(n, pattern[1]))) {
              throw new Error(errorMessage(pattern[1]));
            }
          }
          curNode.patterns.push(pattern);
          parentPatterns.push(...curNode.patterns);
        }
        parentPatterns.push(...curNode.patterns);
        curNode = curNode.children[p];
      }
      if (!curNode.methods.length) {
        curNode.methods = [];
      }
      const m = {};
      const handlerSet = { handler, name: this.name, score: this.order };
      m[method] = handlerSet;
      curNode.methods.push(m);
      return curNode;
    }
    gHSets(node, method, wildcard) {
      var _a, _b;
      return (_a = node.handlerSetCache)[_b = `${method}:${wildcard ? "1" : "0"}`] || (_a[_b] = (() => {
        const handlerSets = [];
        for (let i = 0, len = node.methods.length; i < len; i++) {
          const m = node.methods[i];
          const handlerSet = m[method] || m[METHOD_NAME_ALL];
          if (handlerSet !== void 0) {
            handlerSets.push(handlerSet);
          }
        }
        return handlerSets;
      })());
    }
    search(method, path) {
      const handlerSets = [];
      const params = {};
      const curNode = this;
      let curNodes = [curNode];
      const parts = splitPath(path);
      for (let i = 0, len2 = parts.length; i < len2; i++) {
        const part = parts[i];
        const isLast = i === len2 - 1;
        const tempNodes = [];
        let matched = false;
        for (let j = 0, len22 = curNodes.length; j < len22; j++) {
          const node = curNodes[j];
          const nextNode = node.children[part];
          if (nextNode) {
            if (isLast === true) {
              if (nextNode.children["*"]) {
                handlerSets.push(...this.gHSets(nextNode.children["*"], method, true));
              }
              handlerSets.push(...this.gHSets(nextNode, method));
              matched = true;
            } else {
              tempNodes.push(nextNode);
            }
          }
          for (let k = 0, len3 = node.patterns.length; k < len3; k++) {
            const pattern = node.patterns[k];
            if (pattern === "*") {
              const astNode = node.children["*"];
              if (astNode) {
                handlerSets.push(...this.gHSets(astNode, method));
                tempNodes.push(astNode);
              }
              continue;
            }
            if (part === "")
              continue;
            const [key, name, matcher] = pattern;
            const child = node.children[key];
            const restPathString = parts.slice(i).join("/");
            if (matcher instanceof RegExp && matcher.test(restPathString)) {
              handlerSets.push(...this.gHSets(child, method));
              params[name] = restPathString;
              continue;
            }
            if (matcher === true || matcher instanceof RegExp && matcher.test(part)) {
              if (typeof key === "string") {
                if (isLast === true) {
                  handlerSets.push(...this.gHSets(child, method));
                  if (child.children["*"]) {
                    handlerSets.push(...this.gHSets(child.children["*"], method));
                  }
                } else {
                  tempNodes.push(child);
                }
              }
              if (typeof name === "string" && !matched) {
                params[name] = part;
              } else {
                if (node.children[part]) {
                  params[name] = part;
                }
              }
            }
          }
        }
        curNodes = tempNodes;
      }
      const len = handlerSets.length;
      if (len === 0)
        return null;
      if (len === 1)
        return { handlers: [handlerSets[0].handler], params };
      const handlers = handlerSets.sort((a, b) => {
        return a.score - b.score;
      }).map((s) => {
        return s.handler;
      });
      return { handlers, params };
    }
  };

  // node_modules/hono/dist/router/trie-router/router.js
  var TrieRouter = class {
    constructor() {
      this.name = "TrieRouter";
      this.node = new Node2();
    }
    add(method, path, handler) {
      const results = checkOptionalParameter(path);
      if (results) {
        for (const p of results) {
          this.node.insert(method, p, handler);
        }
        return;
      }
      this.node.insert(method, path, handler);
    }
    match(method, path) {
      return this.node.search(method, path);
    }
  };

  // node_modules/hono/dist/hono.js
  var Hono2 = class extends Hono {
    constructor(init = {}) {
      super(init);
      this.router = init.router ?? new SmartRouter({
        routers: [new RegExpRouter(), new TrieRouter()]
      });
    }
  };

  // index.js
  var app = new Hono2();
  app.get("/", (c) => {
    return c.text("Hello from Hono running in Wasm Workers Server!");
  });
  app.get("/hello", (c) => {
    return c.text("You can get a custom hello message by accessing /hello/your-name");
  });
  app.get("/hello/:name", (c) => {
    const name = c.req.param("name");
    return c.text(`Hello ${name}! This app is running in Wasm Workers Server`);
  });
  app.notFound((c) => {
    return c.text("Awww! This page is missing", 404);
  });
  app.fire();
})();
