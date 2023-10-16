import { Hono } from 'hono'

const app = new Hono()

app.get('/', (c) => {
  return c.text('Hello from Hono running in Wasm Workers Server!')
});

app.get('/hello', (c) => {
  return c.text('You can get a custom hello message by accessing /hello/your-name')
});

app.get('/hello/:name', (c) => {
  const name = c.req.param('name');
  return c.text(`Hello ${name}! This app is running in Wasm Workers Server`)
});

app.notFound((c) => {
  return c.text('Awww! This page is missing', 404)
})

export default app;
