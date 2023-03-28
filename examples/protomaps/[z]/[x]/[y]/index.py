import sys
sys.path.append("/src/libs")
import os

from reader import Reader

# Provide the specific tile from the file
def worker(request):
    path = "/src/maps/{name}.pmtiles".format(
        name=os.environ.get("MAP_FILE")
    )
    map_file = open(path, mode="rb")
    reader = Reader(map_file)

    x = int(request.params["x"])
    y = int(request.params["y"])
    z = int(request.params["z"])

    point = reader.get(z, x , y)

    if point != None:
        return Response(point)
    else:
        return Response("Tile not found")