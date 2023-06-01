# Wasm Workers Server / Project crate

The purpose of this create is to prepare the worker project before we proceed identifying the routes and preparing the individual workers. It's in charge of locating the project locally, pulling it from a supported remote and storing it in a place that it's accessible for `wws`.

It also downloads the required runtimes to run the given project.
