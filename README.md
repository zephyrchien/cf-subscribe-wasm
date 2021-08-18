# Subscription Deliver

## Data Form
Shoud be **POST** to server to register a new endpoint.
>v2ray params (json):
```json
{
  "v": "",
  "ps": "",
  "add": "",
  "port": "",
  "id": "",
  "aid": "",
  "net": "",
  "type": "",
  "host": "",
  "path": "",
  "tls": ""
}
```
>shadowsocks params (json)
```json
{
  "tag": "",
  "server": "",
  "server_port": "",
  "method": "",
  "password": ""
}
```

## Crud APIs
### Path
It is configurable, see `worker/workers.js`
```shell
const config = {
  passwd: 'passwd',
  get_path: '/fetch',
  put_path: '/register',
  list_path: '/list',
  delete_path: '/revoke',
  subscribe_path: '/subscribe'
}
```

### Query Params
| proto | passwd | tag| token |
|:--:|:--:|:--:|:--:|
| must | must | fetch | subscribe |
- proto: ["v2/v2ray", "ss/shadowsocks"]
- passwd: string (could be omitted if token is supplied when pulling subscription)
- tag: string (*optional*)
- token: string (could replace passwd when pulling subscription)

### Curl Examples

#### Put
```shell
curl https://xxx/register?proto=ss&passwd=xxxxxx \
  -X "POST" -H "content-type: application/json" \
  -d '{"tag":"jpss", "server": "xxx"...}'

curl https://xxx/register?proto=v2&passwd=xxxxxx \
  -X "POST" -H "content-type: application/json" \
  -d @v2ray.json
```
> response
```shell
registered: jpss
registered: usv2
```

#### Get
```shell
curl https://xxx/fetch?proto=ss&passwd=xxxxxx&tag=jp
curl https://xxx/fetch?proto=v2&passwd=xxxxxx&tag=us
```
> response
```shell
ss://xx:xx@xx:xx#xx
vmess://xxxxxxxxxxx
```

#### List
```shell
curl https://xxx/list?proto=ss&passwd=xxxxxx
curl https://xxx/list?proto=v2&passwd=xxxxxx
```
> response
```shell
tags: jpss, usss, hkss
tags: jpv2, usv2, hkv2
```

#### Delete
```shell
curl https://xxx/revoke?proto=ss&passwd=xxxxxx&tag=jpss
curl https://xxx/revoke?proto=v2&passwd=xxxxxx&tag=usv2
```
> response
```shell
revoked: jpss
revoked: usv2
```

#### Subscribe
```shell
curl https://xxx/subscribe?proto=ss&passwd=xxxxxx
curl https://xxx/subscribe?proto=v2&token=xxxxxxx
```
>response
```shell
base64(array(ss://xxxxxx))
base64(array(vmess://xxx))
```
