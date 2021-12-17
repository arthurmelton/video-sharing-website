function update_dark_mode() {
    if (localStorage.getItem("DarkMode") !== null) {
        if (localStorage.getItem("DarkMode") == "true") {
            document.documentElement.classList.add("dark");
        }
        else {
            document.documentElement.classList.remove("dark");
        }
    }
    else {
        if (window.matchMedia('(prefers-color-scheme: dark)').matches) {
            localStorage.setItem("DarkMode", "true");
            document.documentElement.classList.add("dark");
        }
        else {
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
    let reader = new FileReader();
    reader.readAsText(document.getElementById('fileInput').files[0]);
    reader.onload = function() {
        console.log(reader.result);
        upload(reader.result);
      };
}

function upload(uploads) {
    $.post("upload", uploads + "\r\n\r\n", function(status) {
        $.get("upload", "done", function(data){
            $("#video_link").attr("href", data);
            $("#done").attr("style", "");
            document.getElementById('uploading').style = "display:none";
        });
    })
}

function change_dark_mode() {
    if (localStorage.getItem("DarkMode") == "true") {
        localStorage.setItem("DarkMode", "false");
    }
    else {
        localStorage.setItem("DarkMode", "true");
    }
    update_dark_mode();
}