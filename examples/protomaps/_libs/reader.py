# Copyright 2021 Protomaps LLC
# License: BSD-3-Clause
#
# Redistribution and use in source and binary forms, with or without modification, are permitted provided that the following conditions are met:
#
# 1. Redistributions of source code must retain the above copyright notice, this list of conditions and the following disclaimer.
#
# 2. Redistributions in binary form must reproduce the above copyright notice, this list of conditions and the following disclaimer in the documentation and/or other materials provided with the distribution.
#
# 3. Neither the name of the copyright holder nor the names of its contributors may be used to endorse or promote products derived from this software without specific prior written permission.
#
# THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
#
# Modifications:
# - Removing mmap dependency and logic
# - Load the data directly from the file

import json
import gzip
from tile import (
    deserialize_header,
    deserialize_directory,
    zxy_to_tileid,
    tileid_to_zxy,
    find_tile
)

class Reader:
    def __init__(self, map_file):
        self.map_file = map_file

    def get_bytes(self, offset, length):
        self.map_file.seek(offset)
        return self.map_file.read(length)

    def header(self):
        return deserialize_header(self.get_bytes(0, 127))

    def metadata(self):
        header = deserialize_header(self.get_bytes(0, 127))
        metadata = self.get_bytes(header["metadata_offset"], header["metadata_length"])
        return json.loads(metadata)

    def get(self, z, x, y):
        tile_id = zxy_to_tileid(z, x, y)
        header = deserialize_header(self.get_bytes(0, 127))
        dir_offset = header["root_offset"]
        dir_length = header["root_length"]
        for depth in range(0, 4):  # max depth
            directory = deserialize_directory(self.get_bytes(dir_offset, dir_length))
            result = find_tile(directory, tile_id)
            if result:
                if result.run_length == 0:
                    dir_offset = header["leaf_directory_offset"] + result.offset
                    dir_length = result.length
                else:
                    return gzip.decompress(self.get_bytes(
                        header["tile_data_offset"] + result.offset, result.length
                    ))


def traverse(get_bytes, header, dir_offset, dir_length):
    entries = deserialize_directory(get_bytes(dir_offset, dir_length))
    for entry in entries:
        if entry.run_length > 0:
            yield tileid_to_zxy(entry.tile_id), get_bytes(
                header["tile_data_offset"] + entry.offset, entry.length
            )
        else:
            for t in traverse(
                get_bytes,
                header,
                header["leaf_directory_offset"] + entry.offset,
                entry.length,
            ):
                yield t


def all_tiles(get_bytes):
    header = deserialize_header(get_bytes(0, 127))
    return traverse(get_bytes, header, header["root_offset"], header["root_length"])
