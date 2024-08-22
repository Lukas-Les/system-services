.PHONY: install_trx_gen uninstall_trx_gen install_print_color uninstall_print_color

install_trx_gen:
	./scripts/install.sh transactions-generator

uninstall_trx_gen:
	./scripts/uninstall.sh transactions-generator

install_print_color:
	./scripts/install.sh print-color

uninstall_print_color:
	./scripts/uninstall.sh print-color
