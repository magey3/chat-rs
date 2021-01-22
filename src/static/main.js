function httpPOST(url, payload){
	var r = new XMLHttpRequest();
	r.open("POST", url, false);
	r.setRequestHeader("Content-Type", "application/json");
	var d = JSON.stringify(payload);
	r.send(d);
	return r.responseText;
}

var data = JSON.parse(httpPOST("https://localhost:8080/json", {index: 1, amount: 1}));
console.log(data);
