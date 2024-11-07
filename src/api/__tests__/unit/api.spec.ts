import { describe, it, expect, vi, afterEach, Mock } from "vitest";
import {
  getSerialPorts,
  openSerialPort,
  closeSerialPort,
  subscribe,
  unsubscribe,
  toggleReadState,
  sendToSerialPort,
  sendToAllSerialPorts,
  SerialVauApi,
} from "@/api/api";
import { invoke } from "@tauri-apps/api";
import {
  DataBits,
  FlowControl,
  OpenSerialPortOptions,
  Parity,
  StopBits,
} from "@/models/open-options";
import { ReadState } from "@/models/managed-serial-port";

vi.mock("@tauri-apps/api", () => ({
  invoke: vi.fn(),
}));

describe("Serial API functions", () => {
  const mockInvoke = invoke as Mock;

  afterEach(() => {
    vi.clearAllMocks();
  });

  it("should get serial ports", async () => {
    const mockPorts = [{ name: "COM1" }];
    mockInvoke.mockResolvedValue(mockPorts);

    const result = await getSerialPorts();
    expect(result).toEqual(mockPorts);
    expect(mockInvoke).toHaveBeenCalledWith(SerialVauApi.GET_SERIAL_PORTS);
  });

  it("should open a serial port with given name and options", async () => {
    const mockResponse = { name: "COM1", status: "open" };
    const options: OpenSerialPortOptions = {
      baudRate: 9600,
      initialReadState: ReadState.Read,
      dataBits: DataBits.Eight,
      flowControl: FlowControl.Hardware,
      parity: Parity.Even,
      stopBits: StopBits.One,
      timeout: { nanos: 1, secs: 10 },
    };
    mockInvoke.mockResolvedValue(mockResponse);

    const result = await openSerialPort("COM1", options);
    expect(result).toEqual(mockResponse);
    expect(mockInvoke).toHaveBeenCalledWith(SerialVauApi.OPEN_SERIAL_PORT, {
      name: "COM1",
      options,
    });
  });

  it("should close a serial port with given name", async () => {
    const mockResponse = { name: "COM1", status: "closed" };
    mockInvoke.mockResolvedValue(mockResponse);

    const result = await closeSerialPort("COM1");
    expect(result).toEqual(mockResponse);
    expect(mockInvoke).toHaveBeenCalledWith(SerialVauApi.CLOSE_SERIAL_PORT, {
      name: "COM1",
    });
  });

  it("should subscribe to events between two sources", async () => {
    const mockResponse = {
      from: "sourceA",
      to: "sourceB",
      status: "subscribed",
    };
    mockInvoke.mockResolvedValue(mockResponse);

    const result = await subscribe("sourceA", "sourceB");
    expect(result).toEqual(mockResponse);
    expect(mockInvoke).toHaveBeenCalledWith(SerialVauApi.SUBSCRIBE, {
      from: "sourceA",
      to: "sourceB",
    });
  });

  it("should unsubscribe from events between two sources", async () => {
    const mockResponse = {
      from: "sourceA",
      to: "sourceB",
      status: "unsubscribed",
    };
    mockInvoke.mockResolvedValue(mockResponse);

    const result = await unsubscribe("sourceA", "sourceB");
    expect(result).toEqual(mockResponse);
    expect(mockInvoke).toHaveBeenCalledWith(SerialVauApi.UNSUBSCRIBE, {
      from: "sourceA",
      to: "sourceB",
    });
  });

  it("should toggle read state for a serial port with given name", async () => {
    const mockResponse = { name: "COM1", status: "readToggled" };
    mockInvoke.mockResolvedValue(mockResponse);

    const result = await toggleReadState("COM1");
    expect(result).toEqual(mockResponse);
    expect(mockInvoke).toHaveBeenCalledWith(SerialVauApi.TOGGLE_READ_STATE, {
      name: "COM1",
    });
  });

  it("should send data to a specific serial port", async () => {
    const mockResponse = { name: "COM1", value: "hello", status: "sent" };
    mockInvoke.mockResolvedValue(mockResponse);

    const result = await sendToSerialPort("COM1", "hello");
    expect(result).toEqual(mockResponse);
    expect(mockInvoke).toHaveBeenCalledWith(SerialVauApi.SEND_TO_SERIAL_PORT, {
      name: "COM1",
      value: "hello",
    });
  });

  it("should send data to all serial ports", async () => {
    const mockResponse = { status: "sentToAll", value: "hello" };
    mockInvoke.mockResolvedValue(mockResponse);

    const result = await sendToAllSerialPorts("hello");
    expect(result).toEqual(mockResponse);
    expect(mockInvoke).toHaveBeenCalledWith(SerialVauApi.SEND_TO_SERIAL_PORTS, {
      value: "hello",
    });
  });
});
