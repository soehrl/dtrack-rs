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
}
