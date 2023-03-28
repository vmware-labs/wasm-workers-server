.PHONY: all build-image

download-map:
	curl -kL --output ./_maps/planet.pmtiles \
		https://pub-9288c68512ed46eca46ddcade307709b.r2.dev/protomaps-sample-datasets/protomaps_vector_planet_odbl_z10.pmtiles

install:
	wws runtimes install

run:
	wws .

build-image:
	docker build -t wws-maps:latest .

all: download-map install run
