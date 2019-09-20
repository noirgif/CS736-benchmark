all: 
	cargo build --release --bin us --bin uc --bin tis --bin tic --bin clock_res --bin kernelcall --bin measure_pipe

test: clock_res systemcall measure_pipe tcpip_lat tcpip_tput udp_lat udp_tput

.PHONY: clock_res
clock_res:
	cargo run --release --bin clock_res

.PHONY: systemcall
systemcall:
	cargo run --release --bin kernelcall

.PHONY: tcpip_lat
tcpip_lat:
	cargo run --release --bin tis latency 127.0.0.1:8082 &
	sleep 1 && cargo run --release --bin tic latency 127.0.0.1:8082

.PHONY: tcpip_tput
tcpip_tput:
	cargo run --release --bin tis throughput 127.0.0.1:8085 &
	sleep 1 && cargo run --release --bin tic throughput 127.0.0.1:8085

.PHONY: udp_lat
udp_lat:
	cargo run --release --bin us lat 10000 127.0.0.1:8086 127.0.0.1:8087 &
	sleep 1 && cargo run --release --bin uc lat 10000 127.0.0.1:8087 127.0.0.1:8086

.PHONY: udp_tput
udp_lat:
	cargo run --release --bin us tput 10000 127.0.0.1:8088 127.0.0.1:8089 &
	sleep 1 && cargo run --release --bin uc tput 10000 127.0.0.1:8089 127.0.0.1:8088

.PHONY: measure_pipe
measure_pipe:
	cargo run --release --bin measure_pipe