const { invoke } = window.__TAURI__.core;

function setCookie(cookieName, value, days) {
    const d = new Date();
    d.setTime(d.getTime() + (days * 24 * 60 * 60 * 1000));

    let expires = "expires=" + d.toUTCString();
    document.cookie = cookieName + "=" + value + ";" + expires + ";path=/";
}
  
function getCookie(cookieName) {
    let name = cookieName + "=";
    let cookies = document.cookie.split(';');

    for (let i = 0; i < cookies.length; i++) {
        let cookie = cookies[i];

        while (cookie.charAt(0) == ' ') {
            cookie = cookie.substring(1);
        }

        if (cookie.indexOf(name) == 0) {
            return cookie.substring(name.length, cookie.length);
        }
    }

    return "";
}

function startSession() {
    let session_id = get_cookie("RUST_SESSION_ID");

    console.log("TEST");


    if (session_id == "") {


        invoke("get_uuid", {})
            .then(uuid => {
                invoke("start_session", { sessionId: uuid })
                    .catch(e => console.error(e));
            });
    }
}