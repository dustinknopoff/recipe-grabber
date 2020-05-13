addEventListener("fetch", (event) => {
  event.respondWith(handleRequest(event.request));
});

/**
 * Fetch and log a request
 * @param {Request} request
 */
async function handleRequest(request) {
  if (get("url", request.url)) {
    const { get_ld_json } = wasm_bindgen;
    await wasm_bindgen(wasm);

    let data = await fetch(get("url", request.url)).then((r) => r.text());

    const recipe_context = get_ld_json(data);

    let res = new Response(recipe_context, { status: 200 });
    // res.headers.set("Content-type", "application/json");
    return res;
  }
  return new Response("ERROR. No Body POST'd", { status: 403 });
}

function get(name, url) {
  if (
    (name = new RegExp("[?&]" + encodeURIComponent(name) + "=([^&]*)").exec(
      url
    ))
  )
    return decodeURIComponent(name[1]);
}
