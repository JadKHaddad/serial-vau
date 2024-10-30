import { listen, TauriEvent } from "@tauri-apps/api/event";
import {
  listenPacketEvent,
  listenSerialPortEvent,
  listenThemeChangedEvent,
  SerialVauEvents,
} from "@/api/listener";
import { beforeEach, describe, expect, it, Mock, vi } from "vitest";

vi.mock("@tauri-apps/api/event", () => ({
  listen: vi.fn(),
  TauriEvent: {
    WINDOW_THEME_CHANGED: "tauri://window-theme-changed",
  },
}));

describe("api/listener", () => {
  const mockListen = listen as Mock;

  beforeEach(() => {
    mockListen.mockClear();
  });

  it("should set up a theme change listener and invoke the handler", async () => {
    const mockHandler = vi.fn();

    const mockRevokeCallback = vi.fn();
    mockListen.mockResolvedValue(mockRevokeCallback);

    await listenThemeChangedEvent(mockHandler);

    expect(mockListen).toHaveBeenCalledWith(
      TauriEvent.WINDOW_THEME_CHANGED,
      expect.any(Function)
    );

    const callback = mockListen.mock.calls[0][1];
    callback({ payload: "dark" });

    expect(mockHandler).toHaveBeenCalledWith({ payload: "dark" });
  });

  it("should set up a serial port event listener and invoke the handler", async () => {
    const mockHandler = vi.fn();

    const mockRevokeCallback = vi.fn();
    mockListen.mockResolvedValue(mockRevokeCallback);

    const revokeCallback = await listenSerialPortEvent(mockHandler);

    expect(mockListen).toHaveBeenCalledWith(
      SerialVauEvents.SERIAL_PORT_EVENT,
      expect.any(Function)
    );

    const callback = mockListen.mock.calls[0][1];
    callback({ payload: "COM1" });

    expect(mockHandler).toHaveBeenCalledWith({ payload: "COM1" });

    revokeCallback();
    expect(mockRevokeCallback).toHaveBeenCalled();
  });

  it("should set up a packet event listener and invoke the handler", async () => {
    const mockHandler = vi.fn();

    const mockRevokeCallback = vi.fn();
    mockListen.mockResolvedValue(mockRevokeCallback);

    const revokeCallback = await listenPacketEvent(mockHandler);

    expect(mockListen).toHaveBeenCalledWith(
      SerialVauEvents.SERIAL_PACKET_EVENT,
      expect.any(Function)
    );

    const callback = mockListen.mock.calls[0][1];
    callback({ payload: { packet: "data" } });

    expect(mockHandler).toHaveBeenCalledWith({ payload: { packet: "data" } });

    revokeCallback();
    expect(mockRevokeCallback).toHaveBeenCalled();
  });
  describe("SerialVaultEvents", () => {
    it("should have the correct values for SerialVauEvents enum", () => {
      expect(SerialVauEvents.SERIAL_PORT_EVENT).toBe("serial_ports_event");
      expect(SerialVauEvents.SERIAL_PACKET_EVENT).toBe("serial_packet_event");
    });
  });
});
