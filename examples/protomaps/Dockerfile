FROM bitnami/minideb:bullseye as builder

# Download the map
RUN install_packages curl
RUN curl -kL --output /planet.pmtiles https://pub-9288c68512ed46eca46ddcade307709b.r2.dev/protomaps-sample-datasets/protomaps_vector_planet_odbl_z10.pmtiles

FROM ghcr.io/vmware-labs/wws:preview

COPY . /app
COPY --from=builder /planet.pmtiles /app/_maps/