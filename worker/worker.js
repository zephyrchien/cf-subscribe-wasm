addEventListener('fetch', event => {
	event.respondWith(handleRequest(event.request))
})

async function handleRequest(request) {
	const { handle } = wasm_bindgen;
	await wasm_bindgen(wasm);
	let response;
	try {
		response = await handle(request, v2ray, shadowsocks);
	} catch (error) {
		response = new Response(JSON.stringify(error), {
			status: 500,
			statusText: 'Internal Server Error',
			headers: {
				'content-type': 'plain/text'
			}
		})
	}
	return response;
}

/**
async function handleRequest(request) {
	const url = new URL(request.url);
	if (url.pathname == '/subscribe' && request.method == 'GET') {
		return await subscribe(url.searchParams);
	}
	if (url.pathname.startsWith('/register') && request.method == 'POST') {
		const sub_path = url.pathname.substring('/register'.length);
		return await register(request, sub_path);
	}
	return not_found();
}


// cloudflare kv bindings
const kv = {
	'get_v2ray': async(k) => {
		return await v2ray.get(k);
	},
	'put_v2ray': async(k, v) => {
		return await v2ray.put(k, v);
	},
	'list_v2ray': async() => {
		return await v2ray.list();
	},
	'get_shadowsocks': async(k) => {
		return await shadowsocks.get(k);
	},
	'put_shadowsocks': async(k, v) => {
		return await shadowsocks.put(k, v);
	},
	'list_shadowsocks': async() => {
		return await shadowsocks.list();
	}
};

function not_found() {
	return new Response(null, {
		status: 404,
		statusText: 'Not Found'
	});
}

function new_response(message) {
	return new Response(message, {
		status: 200,
		statusText: 'OK',
		headers: {
			'content-type': 'plain/text'
		}
	});
}

async function register(request, sub_path) {
	const data = await request.json();
	switch (sub_path) {
		case '/v2ray':
			return await register_v2ray(data);
		case '/shadowsocks':
			return await register_shadowsocks(data);
		default:
			return not_found();
	}
}
**/

/**
v2ray params
*	v, ps,
*	add, port,
*	id, aid,
*	net, type,
*	host, path,
*	tls
**/
async function register_v2ray(data) {
	const tag = data.ps;
	const link = 'vmess://' + base64(JSON.stringify(data));
	await kv.put_v2ray(tag , link);
	return new_response(`${tag} registered`);
}

/**
*	tag,
*	server, server_port,
*	method,
*	password
**/
async function register_shadowsocks(data) {
	const tag = data.tag;
	const server = `${data.server}:${data.server_port}`;
	const account = base64(`${data.method}:${data.password}`);
	const link = `ss://${account}@${server}#${tag}`;
	await kv.put_shadowsocks(tag, link);
	return new_response(`${tag} registered`);
}

/**
async function subscribe(form) {
	const token = form.get('token');
	const proto = form.get('proto');

	const month = new Date().getMonth() + 1;
	const valid_token = md5sum(month.toString());
	if (token != valid_token) {
		return not_found();
	}
	switch (proto) {
		case 'v2':
			return await subscribe_v2ray();
		case 'ss':
			return await subscribe_shadowsocks();
		default:
			return not_found();
	}
}

async function subscribe_v2ray() {
	const list = await kv.list_v2ray();
	const keys = list['keys'];
	let tasks = [];
	for (let i = 0, len = keys.length; i < len; i++) {
		const name = keys[i]['name'];
		tasks.push(kv.get_v2ray(name));
	}
	let res = Promise.all(tasks);
	return new_response((await res).join('\n'));
}

async function subscribe_shadowsocks() {
	const list = await kv.list_shadowsocks();
	const keys = list['keys'];
	let tasks = [];
	for (let i = 0, len = keys.length; i < len; i++) {
		const name = keys[i]['name'];
		tasks.push(kv.get_shadowsocks(name));
	}
	let res = Promise.all(tasks);
	return new_response((await res).join('\n'));
}
**/
