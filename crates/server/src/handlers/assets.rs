// Copyright 2022 VMware, Inc.
// SPDX-License-Identifier: Apache-2.0

use actix_files::NamedFile;
use actix_web::{web::Data, HttpRequest};
use std::{
    io::{Error, ErrorKind},
    path::{Component, Path, PathBuf},
};

/// Clean up invalid components in the paths and returns it. For a file
/// in the public folder, only "normal" components are valid.
fn clean_up_path(uri: &str) -> PathBuf {
    // First split the URI as it always uses the /.
    let path = PathBuf::from_iter(uri.split('/'));

    let valid_components: Vec<Component<'_>> = path
        .components()
        // Keep only normal components. The relative components should be
        // strip by actix, but we're double checking it in case of weird encodings
        // that can be interpreted as parent paths. Note this is a path that will
        // be appended later to the public folder.
        .filter(|c| matches!(c, Component::Normal(_)))
        .collect();

    // Build a new PathBuf based only on valid components
    PathBuf::from_iter(valid_components)
}

/// Build the file path to retrieve and check if it exists. To build, it takes the project
/// root and the parsed path. You can set it the index_folder flag to manage the
/// parsed_path as a folder an look for an index.html inside it.
fn retrieve_asset_path(root_path: &Path, file_path: &Path, index_folder: bool) -> Option<PathBuf> {
    let public_folder = root_path.join("public");
    let asset_path = if index_folder {
        public_folder.join(file_path).join("index.html")
    } else {
        public_folder.join(file_path)
    };

    // Checks the output path is a child of public folder
    if asset_path.starts_with(public_folder) && asset_path.exists() && asset_path.is_file() {
        Some(asset_path)
    } else {
        None
    }
}

/// Find a static HTML file in the `public` folder. This function is used
/// when there's no direct file to be served. It will look for certain patterns
/// like "public/{uri}/index.html" and "public/{uri}.html".
///
/// If no file is present, it will try to get a default "public/404.html"
pub async fn handle_assets(req: &HttpRequest) -> Result<NamedFile, Error> {
    let root_path = req.app_data::<Data<PathBuf>>().unwrap();
    let uri_path = req.path();

    // Double-check the given path path does not contain any unexpected value.
    // It was previously sanitized, but this is a double check.
    let parsed_path = clean_up_path(uri_path);

    if let Some(file_path) = retrieve_asset_path(root_path, &parsed_path, false) {
        // File path. This is required for the wasm_handler as dynamic routes may capture static files
        NamedFile::open_async(file_path).await
    } else if let Some(index_folder_path) = retrieve_asset_path(root_path, &parsed_path, true) {
        // A.k.a pretty urls. We may access /about and this matches to /about/index.html
        NamedFile::open_async(index_folder_path).await
    } else {
        Err(Error::new(ErrorKind::NotFound, "The file is not present"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clean_up_path() {
        let tests = if cfg!(target_os = "windows") {
            Vec::from([
                ("/", PathBuf::new()),
                ("/index.js", PathBuf::from("index.js")),
                ("/my-folder/index.js", PathBuf::from("my-folder\\index.js")),
                // These scenarios are unlikely as actix already filters the
                // URI, but let's test them too
                ("/../index.js", PathBuf::from("index.js")),
                ("/../../index.js", PathBuf::from("index.js")),
            ])
        } else {
            Vec::from([
                ("/", PathBuf::new()),
                ("/index.js", PathBuf::from("index.js")),
                ("////index.js", PathBuf::from("index.js")),
                ("/my-folder/index.js", PathBuf::from("my-folder/index.js")),
                // These scenarios are unlikely as actix already filters the
                // URI, but let's test them too
                ("/../index.js", PathBuf::from("index.js")),
                ("/../../index.js", PathBuf::from("index.js")),
            ])
        };

        for (uri, path) in tests {
            assert_eq!(clean_up_path(uri), path);
        }
    }

    #[test]
    fn relative_asset_path_retrieval() {
        let (project_root, tests) = if cfg!(target_os = "windows") {
            let project_root = Path::new("..\\..\\tests\\data");
            let tests = Vec::from([
                // Existing files
                (
                    Path::new("index.html"),
                    Some(PathBuf::from("..\\..\\tests\\data\\public\\index.html")),
                ),
                (
                    Path::new("main.css"),
                    Some(PathBuf::from("..\\..\\tests\\data\\public\\main.css")),
                ),
                // Missing files
                (Path::new(""), None),
                (Path::new("unknown"), None),
                (Path::new("about"), None),
            ]);

            (project_root, tests)
        } else {
            let project_root = Path::new("../../tests/data");
            let tests = Vec::from([
                // Existing files
                (
                    Path::new("index.html"),
                    Some(PathBuf::from("../../tests/data/public/index.html")),
                ),
                (
                    Path::new("main.css"),
                    Some(PathBuf::from("../../tests/data/public/main.css")),
                ),
                // Missing files
                (Path::new(""), None),
                (Path::new("unknown"), None),
                (Path::new("about"), None),
            ]);

            (project_root, tests)
        };

        for (file, asset_path) in tests {
            assert_eq!(retrieve_asset_path(project_root, file, false), asset_path);
        }
    }

    #[test]
    fn absolute_asset_path_retrieval() {
        let (project_root, tests) = if cfg!(target_os = "windows") {
            let project_root = Path::new("..\\..\\tests\\data").canonicalize().unwrap();
            let tests = Vec::from([
                // Existing files
                (
                    Path::new("index.html"),
                    Some(project_root.join("public\\index.html")),
                ),
                (
                    Path::new("main.css"),
                    Some(project_root.join("public\\main.css")),
                ),
                // Missing files
                (Path::new(""), None),
                (Path::new("unknown"), None),
                (Path::new("about"), None),
            ]);

            (project_root, tests)
        } else {
            let project_root = Path::new("../../tests/data").canonicalize().unwrap();

            let tests = Vec::from([
                // Existing files
                (
                    Path::new("index.html"),
                    Some(project_root.join("public/index.html")),
                ),
                (
                    Path::new("main.css"),
                    Some(project_root.join("public/main.css")),
                ),
                // Missing files
                (Path::new(""), None),
                (Path::new("unknown"), None),
                (Path::new("about"), None),
            ]);

            (project_root, tests)
        };

        for (file, asset_path) in tests {
            assert_eq!(retrieve_asset_path(&project_root, file, false), asset_path);
        }
    }

    #[test]
    fn relative_asset_index_path_retrieval() {
        let (project_root, tests) = if cfg!(target_os = "windows") {
            let project_root = Path::new("..\\..\\tests\\data");
            let tests = Vec::from([
                // Existing index files
                (
                    Path::new("about"),
                    Some(PathBuf::from(
                        "..\\..\\tests\\data\\public\\about\\index.html",
                    )),
                ),
                (
                    Path::new(""),
                    Some(PathBuf::from("..\\..\\tests\\data\\public\\index.html")),
                ),
                // Missing index files
                (Path::new("main.css"), None),
                (Path::new("unknown"), None),
            ]);

            (project_root, tests)
        } else {
            let project_root = Path::new("../../tests/data");
            let tests = Vec::from([
                // Existing index files
                (
                    Path::new("about"),
                    Some(PathBuf::from("../../tests/data/public/about/index.html")),
                ),
                (
                    Path::new(""),
                    Some(PathBuf::from("../../tests/data/public/index.html")),
                ),
                // Missing index files
                (Path::new("main.css"), None),
                (Path::new("unknown"), None),
            ]);

            (project_root, tests)
        };

        for (file, asset_path) in tests {
            assert_eq!(retrieve_asset_path(project_root, file, true), asset_path);
        }
    }

    #[test]
    fn absolute_asset_index_path_retrieval() {
        let (project_root, tests) = if cfg!(target_os = "windows") {
            let project_root = Path::new("..\\..\\tests\\data").canonicalize().unwrap();
            let tests = Vec::from([
                // Existing idnex files
                (
                    Path::new("about"),
                    Some(project_root.join("public\\about\\index.html")),
                ),
                (Path::new(""), Some(project_root.join("public\\index.html"))),
                // Missing index files
                (Path::new("main.css"), None),
                (Path::new("unknown"), None),
            ]);

            (project_root, tests)
        } else {
            let project_root = Path::new("../../tests/data").canonicalize().unwrap();

            let tests = Vec::from([
                // Existing index files
                (
                    Path::new("about"),
                    Some(project_root.join("public/about/index.html")),
                ),
                (Path::new(""), Some(project_root.join("public/index.html"))),
                // Missing index files
                (Path::new("main.css"), None),
                (Path::new("unknown"), None),
            ]);

            (project_root, tests)
        };

        for (file, asset_path) in tests {
            assert_eq!(retrieve_asset_path(&project_root, file, true), asset_path);
        }
    }
}
