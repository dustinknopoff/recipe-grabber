addEventListener("fetch", (event) => {
  event.respondWith(handleRequest(event.request));
});

/**
 * Fetch and log a request
 * @param {Request} request
 */
async function handleRequest(request) {
  let url = get("url", request.url);
  if (url) {
    const { get_ld_json } = wasm_bindgen;
    await wasm_bindgen(wasm);

    let data = await fetch(url).then((r) => r.text());

    const recipe_context = `${get_ld_json(data)}(${url})`;

    let res = new Response(recipe_context, { status: 200 });
    return res;
  }
  return new Response(
    "ERROR. No url passed to perform conversion to markdown",
    { status: 400 }
  );
}

function get(name, url) {
  if (
    (name = new RegExp("[?&]" + encodeURIComponent(name) + "=([^&]*)").exec(
      url
    ))
  )
    return decodeURIComponent(name[1]);
}
