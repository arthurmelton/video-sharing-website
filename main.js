function update_dark_mode() {
    if (localStorage.getItem("DarkMode") !== null) {
        if (localStorage.getItem("DarkMode") == "true") {
            document.documentElement.classList.add("dark");
            $('.sun').css('display', 'none');
            $('.circle-2').css('display', 'none');
            $('.moon').css('display', 'block');
            $('.circle-1').css('display', 'block');
        }
        else {
            document.documentElement.classList.remove("dark");
            $('.sun').css('display', 'block');
            $('.circle-2').css('display', 'block');
            $('.moon').css('display', 'none');
            $('.circle-1').css('display', 'none');
        }
    }
    else {
        if (window.matchMedia('(prefers-color-scheme: dark)').matches) {
            localStorage.setItem("DarkMode", "true");
            document.documentElement.classList.add("dark");
            $('.sun').css('display', 'none');
            $('.circle-2').css('display', 'none');
            $('.moon').css('display', 'block');
            $('.circle-1').css('display', 'block');
        }
        else {
            localStorage.setItem("DarkMode", "false");
            document.documentElement.classList.remove("dark");
            $('.sun').css('display', 'block');
            $('.circle-2').css('display', 'block');
            $('.moon').css('display', 'none');
            $('.circle-1').css('display', 'none');
        }
    }
}

update_dark_mode();

function init() {
    document.getElementById('fileInput').addEventListener('change', handleFileSelect, false);
    if (localStorage.getItem("DarkMode") == "true") {
        $('.sun').css('display', 'none');
        $('.circle-2').css('display', 'none');
        $('.moon').css('display', 'block');
        $('.circle-1').css('display', 'block');
    }
    else {
        $('.sun').css('display', 'block');
        $('.circle-2').css('display', 'block');
        $('.moon').css('display', 'none');
        $('.circle-1').css('display', 'none');
    }
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
    request.addEventListener('load', function(e) {
        $.get("upload", "done", function(data){
            window.location.href = data;
        });
    });
    request.send(formData);
}

function progressHandling(event) {
    var percent = 0;
    var position = event.loaded || event.position;
    var total = event.total;
    if (event.lengthComputable) {
        percent = Math.round(position / total * 100);
    }
    $("#uploading").text("Uploading your video please wait! ("+percent+"%)");
};

function change_dark_mode() {
    if (localStorage.getItem("DarkMode") == "true") {
        localStorage.setItem("DarkMode", "false");
    }
    else {
        localStorage.setItem("DarkMode", "true");
    }
    update_dark_mode();
}