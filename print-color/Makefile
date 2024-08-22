app_name = print-color
service_name = $(app_name).service

install:
	cargo build --release
	sudo cp target/release/$(app_name) /usr/local/bin/
	sudo cp systemd/$(service_name) /etc/systemd/system/
	sudo systemctl daemon-reload
	sudo systemctl enable $(service_name)
	sudo systemctl start $(service_name)

uninstall:
	sudo systemctl stop $(service_name)
	sudo systemctl disable $(service_name)
	sudo rm /usr/local/bin/$(app_name)
	sudo rm /etc/systemd/system/$(service_name)
	sudo systemctl daemon-reload
	sudo systemctl reset-failed
