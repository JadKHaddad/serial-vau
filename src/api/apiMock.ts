// src/apiMock.ts - Extended Mock for @tauri-apps/api and @tauri-apps/api/event
import { mockManagedSerialPorts } from "@/api/mockData"; // Import mock data
import { StatusType, ReadState } from "@/models/managed-serial-port";

export const invoke = (cmd: string, args?: Record<string, any>) => {
  return new Promise((resolve, reject) => {
    switch (cmd) {
      case "get_serial_ports":
        resolve(mockManagedSerialPorts);
        break;
      case "open_serial_port":
        // Find the port by name in the mock data and update its status
        const portToOpen = mockManagedSerialPorts.findIndex(
          (port) => port.name === args?.name
        );

        if (portToOpen !== -1) {
          mockManagedSerialPorts[portToOpen].status = {
            type: StatusType.Open,
            content: { readState: ReadState.Read },
          };
          resolve(mockManagedSerialPorts);
        } else {
          reject(new Error(`Port ${args?.name} not found`));
        }
        break;
      case "close_serial_port":
        // Find the port by name in the mock data and update its status
        const portToClose = mockManagedSerialPorts.findIndex(
          (port) => port.name === args?.name
        );
        if (portToClose !== -1) {
          mockManagedSerialPorts[portToClose].status = {
            type: StatusType.Closed,
          };

          resolve(mockManagedSerialPorts);
        } else {
          reject(new Error(`Port ${args?.name} not found`));
        }
        break;
      case "subscribe":
      case "unsubscribe":
        // Mock response for subscribe/unsubscribe
        const port = mockManagedSerialPorts.find(
          (port) => port.name === args?.from
        );
        if (port) {
          if (cmd === "subscribe") {
            port.subscriptions.push(args?.to); // Add subscription
          } else {
            port.subscriptions = port.subscriptions.filter(
              (sub) => sub !== args?.to
            ); // Remove subscription
          }
          resolve([port]);
        } else {
          reject(new Error(`Port ${args?.from} not found`));
        }
        break;
      case "toggle_read_state":
        // Toggle the read state for the specified port
        const portToToggle = mockManagedSerialPorts.find(
          (port) => port.name === args?.name
        );
        if (portToToggle) {
          resolve([portToToggle]);
        } else {
          reject(new Error(`Port ${args?.name} not found`));
        }
        break;
      case "send_to_serial_port":
      case "send_to_all_serial_ports":
        // Mock implementation for sending data
        resolve({});
        break;
      default:
        reject(new Error("Unknown command"));
        break;
    }
  });
};

// Mock Event Listeners

export type UnlistenFn = () => void;

const mockEventListeners: Record<string, Function[]> = {};

// Mock listen function to simulate event handling
export const listen = (
  eventName: string,
  callback: (event: any) => void
): Promise<UnlistenFn> => {
  if (!mockEventListeners[eventName]) {
    mockEventListeners[eventName] = [];
  }

  // Add the callback to the mock listener
  mockEventListeners[eventName].push(callback);

  // Return a mock UnlistenFn that removes the callback
  const unlisten: UnlistenFn = () => {
    mockEventListeners[eventName] = mockEventListeners[eventName].filter(
      (fn) => fn !== callback
    );
  };

  return Promise.resolve(unlisten);
};

// Mock function to trigger events manually in the mock environment
export const triggerEvent = (eventName: string, eventData: any) => {
  if (mockEventListeners[eventName]) {
    mockEventListeners[eventName].forEach((callback) =>
      callback({ payload: eventData })
    );
  }
};
