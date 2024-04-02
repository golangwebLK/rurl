# 使用方式
> rurl http://host:port -X 请求方法（GET等） -H "请求头" -d 数据

> 例如：rurl http://localhost:8080 -X GET -H "content-type: ***" -d '{"name":"lk"}'

> rurl -X POST http://127.0.0.1:5000/upload -F "file=@/Users/RustroverProjects/rurl/README.md" 
 
> rurl -X POST http://127.0.0.1:5000/form -F "username=1&password=2" 
