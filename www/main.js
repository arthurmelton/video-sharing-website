function update_dark_mode() {
    if (localStorage.getItem("DarkMode") !== null) {
        if (localStorage.getItem("DarkMode") == "true") {
            document.documentElement.classList.add("dark");
        } else {
            document.documentElement.classList.remove("dark");
        }
    } else {
        if (window.matchMedia('(prefers-color-scheme: dark)').matches) {
            localStorage.setItem("DarkMode", "true");
            document.documentElement.classList.add("dark");
        } else {
            localStorage.setItem("DarkMode", "false");
            document.documentElement.classList.remove("dark");
        }
    }
}

update_dark_mode();

function init() {
    document.getElementById('fileInput').addEventListener('change', handleFileSelect, false);
}

function handleFileSelect(event) {
    document.getElementById('file').style = "display:none";
    document.getElementById('uploading').style = "";
    upload(document.getElementById('fileInput').files[0]);
}

function upload(uploads) {
    let formData = new FormData();
    formData.append("video", uploads);
    let request = new XMLHttpRequest();
    request.open('POST', '/upload');
    request.upload.addEventListener('progress', progressHandling, false);
    request.onreadystatechange = function() { //Call a function when the state changes.
        if (request.readyState == 4 && request.status == 200) {
            window.location.href = `/video/${request.responseText}`;
        }
    }
    request.send(formData);
}

function progressHandling(event) {
    var percent = 0;
    var position = event.loaded || event.position;
    var total = event.total;
    if (event.lengthComputable) {
        percent = Math.round(position / total * 100);
    }
    $("#uploading").text("Uploading your video please wait! (" + percent + "%)");
};

function change_dark_mode() {
    if (localStorage.getItem("DarkMode") == "true") {
        localStorage.setItem("DarkMode", "false");
    } else {
        localStorage.setItem("DarkMode", "true");
    }
    update_dark_mode();
}
