build:
	cargo build --release --bin us --bin uc --bin tis --bin tic --bin clock_res --bin kernelcall --bin measure_pipe

test: clock_res systemcall pipe_lat pipe_tput tcpip_lat tcpip_tput udp_lat udp_tput

.PHONY: clock_res
clock_res: build
	cargo run --bin clock_res

.PHONY: systemcall
systemcall: build
	cargo run --release --bin kernelcall

.PHONY: tcpip_lat
tcpip_lat: build
	cargo run --release --bin tis latency 127.0.0.1:8082 &
	sleep 1 && cargo run --release --bin tic latency 127.0.0.1:8082

.PHONY: tcpip_tput
tcpip_tput: build
	cargo run --release --bin tis throughput 127.0.0.1:8085 &
	sleep 1 && cargo run --release --bin tic throughput 127.0.0.1:8085

.PHONY: udp_lat
udp_lat: build
	cargo run --release --bin us lat 100000 127.0.0.1:8086 127.0.0.1:8087 &
	sleep 1 && cargo run --release --bin uc lat 100000 127.0.0.1:8087 127.0.0.1:8086

.PHONY: udp_tput
udp_tput: build
	cargo run --release --bin us tput 100000 127.0.0.1:8088 127.0.0.1:8089 &
	sleep 1 && cargo run --release --bin uc tput 100000 127.0.0.1:8089 127.0.0.1:8088

.PHONY: pipe_tput
pipe_tput: build
	cargo run --release --bin measure_pipe tput
		
.PHONY: pipe_lat
pipe_lat:
	cargo run --release --bin measure_pipe tput
