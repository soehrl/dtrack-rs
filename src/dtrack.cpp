#include "DTrackSDK.hpp"
#include <iostream>

extern "C" {

void *sdk_new_with_connection(const char *connection) {
  return new DTrackSDK(connection);
}
void *sdk_new_with_data_port(unsigned short data_port) {
  return new DTrackSDK(data_port);
}
void *sdk_new_with_server_host_data_port(const char *server_host,
                                         unsigned short data_port) {
  return new DTrackSDK(server_host, data_port);
}
void *sdk_new_with_server_host_port_data_port(const char *server_host,
                                              unsigned short server_port,
                                              unsigned short data_port) {
  return new DTrackSDK(server_host, server_port, data_port);
}
void sdk_delete(void *sdk) { delete (DTrackSDK *)sdk; }

bool sdk_is_valid(void *sdk) { return ((DTrackSDK *)sdk)->isValid(); }

bool sdk_is_data_interface_valid(void *sdk) {
  return ((DTrackSDK *)sdk)->isDataInterfaceValid();
}

unsigned short sdk_get_data_port(void *sdk) {
  return ((DTrackSDK *)sdk)->getDataPort();
}

bool sdk_is_command_interface_valid(void *sdk) {
  return ((DTrackSDK *)sdk)->isCommandInterfaceValid();
}

bool sdk_is_command_interface_full_access(void *sdk) {
  return ((DTrackSDK *)sdk)->isCommandInterfaceFullAccess();
}

int sdk_get_remote_system_type(void *sdk) {
  return ((DTrackSDK *)sdk)->getRemoteSystemType();
}

bool sdk_set_data_timeout_us(void *sdk, int timeout_us) {
  return ((DTrackSDK *)sdk)->setDataTimeoutUS(timeout_us);
}

bool sdk_set_command_timeout_us(void *sdk, int timeout_us) {
  return ((DTrackSDK *)sdk)->setCommandTimeoutUS(timeout_us);
}

bool sdk_set_controller_timeout_us(void *sdk, int timeout_us) {
  return ((DTrackSDK *)sdk)->setControllerTimeoutUS(timeout_us);
}

bool sdk_receive(void *sdk) { return ((DTrackSDK *)sdk)->receive(); }

bool sdk_start_measurement(void *sdk) {
  return ((DTrackSDK *)sdk)->startMeasurement();
}

bool sdk_stop_measurement(void *sdk) {
  return ((DTrackSDK *)sdk)->stopMeasurement();
}

}
