# Protomaps + Wasm Workers Server

This example runs an entire Maps Service API based on [Protomaps](https://protomaps.com/). It's a serverless system for serving maps. Based on their official website:

> An alternative to map APIs at 1% the cost, via single static files on your own cloud storage. Deploy datasets like OpenStreetMap for your site in minutes.

## Prerequisites

* Wasm Workers Server (wws):

  ```shell-session
  curl -fsSL https://workers.wasmlabs.dev/install | bash
  ```

## TL;DR

Run `make all`

## Install a map

Before running this example, you need to download a map. You have two options:

* Download the default planet-scale map from the [Protomaps site](https://app.protomaps.com/store/planet-z10) ([download link](https://pub-9288c68512ed46eca46ddcade307709b.r2.dev/protomaps-sample-datasets/protomaps_vector_planet_odbl_z10.pmtiles)).
* Create your map with their [small map creation tool](https://app.protomaps.com/downloads/small_map)

Once you have the map, place it in the `_maps` folder.

## Run the example

To run the example, follow these steps:

1. Install the Python runtime:

    ```plain
    wws runtimes install
    ```

1. Edit the `./[z]/[x]/[y]/index.toml` file and replace the `MAP_FILE` environment with your map filename

1. Run `wws`:

    ```plain
    wws
    ```

## Build a container image

You can deploy this project as a standalone server (`wws`) or you can build the container image. Note that the image will download the [default Protomaps map](https://app.protomaps.com/store/planet-z10), so you may want to edit it to sue your map.

```plain
make build-image
```

# Maps license

The example map (`./_maps/map.pmtiles`) was created with the ["Protomaps Small Map"](https://app.protomaps.com/downloads/small_map) tool based on the OpenStreetData data.

OpenStreetMap® is open data, licensed under the Open Data Commons Open Database License (ODbL) by the OpenStreetMap Foundation (OSMF). Read more at https://www.openstreetmap.org/copyright and https://opendatacommons.org/licenses/odbl/1-0/.

# Resources

* [Mount folders](https://workers.wasmlabs.dev/docs/features/mount-folders)
* [Python documentation](https://workers.wasmlabs.dev/docs/languages/python)
