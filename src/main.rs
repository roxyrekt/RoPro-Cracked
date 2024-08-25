import { serve } from "https://deno.land/std@0.170.0/http/mod.ts";
import { sleep } from "https://deno.land/x/sleep@v1.2.1/mod.ts";

const DataFetchURL = "https://raw.githubusercontent.com/Stefanuk12/RoProPatcher/proxy/data.json";
let Data = { "PHPSESSID": "", "tier": "pro_tier" };

async function reqHandler(req: Request) {
    const RoProURL = new URL(req.url);
    const FoundAPI = RoProURL.pathname.indexOf("///api");
    RoProURL.host = FoundAPI != -1 ? "api.ropro.io" : "ropro.io";
    if (FoundAPI != -1) RoProURL.pathname = RoProURL.pathname.substring(0, FoundAPI);

    const CORSheaders = new Headers();
    const origin = req.headers.get("origin") || "chrome-extension://adbacgifemdbhdkfppmeilbgppmhaobf";

    let AllowedHeaders = "";
    for (const header of req.headers.keys()) {
        AllowedHeaders += header + ", ";
    }
    AllowedHeaders += "ropro-id, ropro-verification";

    CORSheaders.set("access-control-allow-origin", origin);
    CORSheaders.set("access-control-allow-headers", AllowedHeaders);
    CORSheaders.set("access-control-allow-credentials", "true");

    if (req.method.toUpperCase() == "OPTIONS") {
        return new Response(null, { headers: CORSheaders });
    }

    if (RoProURL.pathname == "/getSubscription.php") {
        return new Response(Data.tier, { status: 200, headers: CORSheaders });
    }

    const headers = new Headers(req.headers);
    if (Data.PHPSESSID != "") {
        headers.set("Cookie", `PHPSESSID=${Data.PHPSESSID}`);
    }

    const response = await fetch(RoProURL, {
        method: req.method,
        headers: headers,
        body: req.body
    });

    const responseHeaders = new Headers(response.headers);
    responseHeaders.set("access-control-allow-origin", origin);
    responseHeaders.set("access-control-allow-headers", AllowedHeaders);
    responseHeaders.set("access-control-allow-credentials", "true");

    return new Response(response.body, {
        headers: responseHeaders,
        status: response.status,
        statusText: response.statusText
    });
}

serve(reqHandler, { port: 443 });

async function AlertRoPro() {
    await fetch(`https://api.ropro.io/handleRoProAlert.php?timestamp=${Math.round(Date.now() / 1000)}`, {
        headers: { Cookie: `PHPSESSID=${Data.PHPSESSID}` }
    }).catch(() => console.error("Unable to alert RoPro"));
}

(async () => {
    while (true) {
        Data = await (await fetch(DataFetchURL)).json();
        console.info("Refreshed data", Data);
        await AlertRoPro();
        await sleep(300);
    }
})();
