# -*- mode: ruby -*-
# vi: set ft=ruby :

Vagrant.configure("2") do |config|
    hostname = "kensaku.box"
    locale = "en_GB.UTF.8"

    # Box
    config.vm.box = "ubuntu/trusty64"

    # Shared folders
    config.vm.synced_folder ".", "/srv"

    # Setup
    config.vm.provision :shell, :inline => "touch .hushlogin"
    config.vm.provision :shell, :inline => "hostname #{hostname} && locale-gen #{locale}"
    config.vm.provision :shell, :inline => "apt-get update --fix-missing"
    config.vm.provision :shell, :inline => "apt-get install -q -y g++ make git curl vim"

    # Lang
    config.vm.provision :shell, :inline => "curl -s https://static.rust-lang.org/rustup.sh | sudo sh"
end