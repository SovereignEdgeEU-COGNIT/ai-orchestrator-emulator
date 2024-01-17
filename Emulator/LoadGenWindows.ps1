
$last_host = ((Invoke-WebRequest "http://192.168.1.86:8000/hosts").content.split("{").split("}"))[-2]
$last_host

$_ = Invoke-WebRequest "http://192.168.1.86:8000/start" -Method Post -ContentType 'application/json' -Body "{`"host`":{$last_host}, `"flavors`": [`"cpu`"]}"