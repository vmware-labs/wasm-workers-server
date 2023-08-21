document.getElementById('file')
  .addEventListener('change', getFile)

function getFile(event) {
  const input = event.target
  if ('files' in input && input.files.length > 0) {
    runInference(input.files[0])
  }
}

function runInference(file) {
  readFileContent(file).then(content => {
    let result = btoa(content);

    // Set the image
    document
      .getElementById("image")
      .src = `data:image/jpeg;base64,${result}`;

    fetch("/inference", {
      method: "POST",
      body: result
    }).then(res => res.json())
      .then(json => setResults(json));

  }).catch(error => console.log(error))
}

function setResults(json) {
  for (let i = 0; i < 5; i++) {
    let value = json.data[i];
    let res = document.getElementById(`result-${i}`);
    let progress = res.querySelector(".result_progress");
    let label = res.querySelector(".result_label");

    label.textContent = value[0];
    progress.style.setProperty("--progress", `${Math.max(value[1] * 100, 1)}%`);
  }
}

function readFileContent(file) {
  const reader = new FileReader()
  return new Promise((resolve, reject) => {
    reader.onload = event => resolve(event.target.result)
    reader.onerror = error => reject(error)
    reader.readAsBinaryString(file)
  })
}
