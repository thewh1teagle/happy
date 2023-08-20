/* eslint-disable no-unused-vars */
// @ts-ignore
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";
import _ from "lodash";
import { useEffect, useState } from "react";

// const api = axios.create({ baseURL: "http://127.0.0.1:8573" });

function cx(...cns: (boolean | string | undefined)[]): string {
  return cns.filter(Boolean).join(" ");
}

function hexToRgb(hex: string) {
  // Expand shorthand form (e.g. "03F") to full form (e.g. "0033FF")
  const shorthandRegex = /^#?([a-f\d])([a-f\d])([a-f\d])$/i;
  hex = hex.replace(
    shorthandRegex,
    (_: string, r: string, g: string, b: string) => {
      return r + r + g + g + b + b;
    }
  );

  const result = /^#?([a-f\d]{2})([a-f\d]{2})([a-f\d]{2})$/i.exec(hex);
  return result
    ? {
        r: parseInt(result[1], 16),
        g: parseInt(result[2], 16),
        b: parseInt(result[3], 16),
      }
    : null;
}

function App() {
  const [devices, setDevices] = useState<any>([]);
  const [scanning, setScanning] = useState(false);
  const [connecting, setConnecting] = useState(false);
  const [connected, setConnected] = useState(false);
  const [modes, setModes] = useState<any>({});
  const [power, setPower] = useState(false);
  const [selectedDev, setSelectedDev] = useState<any>({});
  const [color, setColor] = useState("");

  useEffect(() => {
    async function initApp() {
      setTimeout(async () => await invoke('show_main_window'), 100) // wait for render and prevent white screen
      const modes = await invoke('get_modes')
      setModes(modes)
      // const res = await api.get("/modes");
      // setModes(res.data);
    }
    initApp();
  }, []);

  useEffect(() => {
    const debouncedColorChange = _.debounce(async (newColor: any) => {
      // Make your HTTP request here to change the color
      const data = hexToRgb(newColor);
      if (data) {
        await invoke('set_rgb', data)
        // await api.post("/set_rgb", data);
      }
    }, 5); // Debounce for 1 second

    debouncedColorChange(color);

    return () => {
      debouncedColorChange.cancel();
    };
  }, [color]);

  async function scan() {
    setScanning(true);
    const devices: any = await invoke('scan', {})
    setDevices(devices)
    setSelectedDev(devices?.[0].address)
    setScanning(false);
  }

  async function connect() {
    setConnecting(true);
    await invoke('connect', {address: selectedDev})
    // await api.post("/connect", { address: selectedDev });
    setConnected(true);
    setConnecting(false);
  }
  async function disconnect() {
    await invoke('disconnect')
    // await api.post("/disconnect");
    setConnected(false);
  }

  async function setLedPower(state: boolean) {
    // await api.post("/set_power", { state });
    await invoke('set_power', {state})
    setPower(state);
  }


  return (
    <div className="w-full h-full">
      <div className="flex flex-col w-[300px] gap-3 m-auto mt-5">
        {!connected && (
          <button className="btn" onClick={scan}>
            <svg
              xmlns="http://www.w3.org/2000/svg"
              fill="none"
              viewBox="0 0 24 24"
              strokeWidth={1.5}
              stroke="currentColor"
              className={cx("w-6 h-6", scanning && "animate-spin")}
            >
              <path
                strokeLinecap="round"
                strokeLinejoin="round"
                d="M16.023 9.348h4.992v-.001M2.985 19.644v-4.992m0 0h4.992m-4.993 0l3.181 3.183a8.25 8.25 0 0013.803-3.7M4.031 9.865a8.25 8.25 0 0113.803-3.7l3.181 3.182m0-4.991v4.99"
              />
            </svg>
            Scan
          </button>
        )}
        {!connected && (
          <select
            disabled={scanning || devices.length < 1}
            onChange={(e) => setSelectedDev(e.target.value)}
            className="select w-full max-w-xs"
          >
            {devices.length < 1 && (
              <option disabled selected>
                Select Device
              </option>
            )}

            {devices.map((d: any) => (
              <option key={d.address} value={d.address}>
                {d.name}
              </option>
            ))}
          </select>
        )}
        <button
          disabled={scanning || devices.length < 1}
          className={cx("btn btn-primary")}
          onClick={connected ? disconnect : connect}
        >
          {!connecting && connected ? "Disconnect" : !connecting && "Connect"}
          {connecting && <span className="loading loading-spinner"></span>}
        </button>
        {connected && (
          <>
            <button
              className={cx("btn btn-circle btn-outline self-center mt-5 hover:bg-transparent hover:text-current")}
              onClick={() => setLedPower(!power)}
            >
              <svg
                xmlns="http://www.w3.org/2000/svg"
                fill="none"
                viewBox="0 0 24 24"
                strokeWidth={1.5}
                stroke="currentColor"
                className="w-6 h-6 "
              >
                <path
                  strokeLinecap="round"
                  strokeLinejoin="round"
                  d="M5.636 5.636a9 9 0 1012.728 0M12 3v9"
                />
              </svg>
            </button>
            <div className="mt-8 text-start">
              <span className="">Pick color</span>
              <input
                type="color"
                onChange={(e) => setColor(e.target.value)}
                className="w-full h-[50px] mt-3"
              ></input>
            </div>
            <div className="mt-8 text-start">
              <span className="">Set Mode</span>
              <select
                onChange={async (e) => {
                  await invoke('set_mode', {mode: Number(e.target.value)})
                  // await api.post("/set_mode", {
                  //   value: Number(e.target.value),
                  // });
                }}
                className="select w-full max-w-xs mt-3"
              >
                <option disabled selected>
                  Select Mode
                </option>
                {modes && modes?.map?.((m: any) => (
                  <option key={m.name} value={m.value}>
                    {m.name}
                  </option>
                ))}
              </select>
            </div>
          </>
        )}
      </div>
    </div>
  );
}

export default App;