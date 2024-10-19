import {
  ManagedSerialPort,
  ReadState,
  StatusType,
} from "@/models/managed-serial-port";
import { DataBits, FlowControl, Parity, StopBits } from "@/models/open-options";

// Sample OpenSerialPortOptions for mock data
export const mockManagedSerialPorts: ManagedSerialPort[] = [
  {
    name: "COM1",
    status: { type: StatusType.Closed },
    subscriptions: [],
    subscribedTo: [],
    lastUsedOpenOptions: {
      initialReadState: ReadState.Stop,
      baudRate: 115200,
      dataBits: DataBits.Eight,
      flowControl: FlowControl.None,
      parity: Parity.None,
      stopBits: StopBits.One,
      timeout: { secs: 0, nanos: 0 },
    },
  },
  {
    name: "COM2",
    status: { type: StatusType.Closed },
    subscriptions: [],
    subscribedTo: [],
    lastUsedOpenOptions: {
      initialReadState: ReadState.Stop,
      baudRate: 115200,
      dataBits: DataBits.Eight,
      flowControl: FlowControl.None,
      parity: Parity.None,
      stopBits: StopBits.One,
      timeout: { secs: 0, nanos: 0 },
    },
  },
];
