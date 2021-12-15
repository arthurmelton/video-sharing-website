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
    const reader = new FileReader()
    reader.onload = handleFileLoad;
    reader.readAsText(event.target.files[0])
}

function handleFileLoad(event) {
    document.getElementById('fileInput').style = "display:none";
    var sections = event.target.result.match(/[\s\S]{1,45000}/g)
    upload(sections, 0);
}

function upload(sections, int) {
    if (int<sections.length) {
        const promise1 = new Promise((resolve) => {
            $.post("upload", sections[int], function(status) {
                resolve();
            })
        })
        promise1.then(() => {
            upload(sections, int+=1);
        })
    }
    else {
        $.get("upload", "done", function(data){
            $("#video_link").attr("href", data);
            $("#done").attr("style", "");
        });
    }
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