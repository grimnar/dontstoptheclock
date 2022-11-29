let last = last_timestamp();

function subscribe(uri) {
  var retryTime = 1;

  function connect(uri) {
    const events = new EventSource(uri);

	// console.log("received this:", events);
    events.addEventListener("message", (ev) => {
      // console.log("raw data", JSON.stringify(ev.data));
      // console.log("decoded data", JSON.stringify(JSON.parse(ev.data)));
      const msg = JSON.parse(ev.data);
	  var ts = msg["last_stop_ts"];
      if (!"last_stop_ts" in msg) return;
      last = msg["last_stop_ts"]; 
	  console.log("Received: " + last); 
    });

    events.addEventListener("open", () => {
      // console.log(`connected to event stream at ${uri}`);
      retryTime = 1;
    });

    events.addEventListener("error", () => {
      events.close();

      let timeout = retryTime;
      retryTime = Math.min(64, retryTime * 2);
      console.log(`connection lost. attempting to reconnect in ${timeout}s`);
      setTimeout(() => connect(uri), (() => timeout * 1000)());
    });
  }

  connect(uri);
}


function stop_the_clock() {
	var ts = Math.floor(new Date().getTime() / 1000);
	fetch('stop/' + ts, {
		method: 'GET'
	}).then((response) => {
        if (response.ok) console.log("SOMEONE STOPPED THE CLOCK");
      });
}

function time_since(since)  {
      since = new Date(since * 1000);
      now = new Date();
      time = now - since;
      days = Math.floor(time/8.64e7);
      hours = Math.floor((time - (days * 8.64e7))/3.6e6);
      minutes = Math.floor((time - (days * 8.64e7 + hours * 3.6e6)) / 60000);
      seconds = Math.floor((time - (days * 8.64e7 + hours * 3.6e6 + minutes * 60000))/ 1000);

      msg = "";
      if (days > 0) {
        msg = msg + days + " days ";
      }
      
      if (hours > 0) {
        msg = msg + hours + " hours ";
      }
      
      if (minutes > 0) {
        msg = msg + minutes + " minutes ";
      }
      
      msg = msg + seconds + " seconds";
      return msg
}

function last_timestamp() {
	var inputs = document.getElementsByTagName("input");
	var arr = [...inputs].map(i => parseInt(i.value));
	var result = Math.max(...arr);
	return result;

}

subscribe("/stop_events");

let msg;
setInterval(() => {msg = time_since(last);
  document.getElementById('time').innerHTML = msg;
}, 1000);
