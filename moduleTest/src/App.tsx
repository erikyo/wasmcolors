import init, {closest} from '../../pkg/wasmcolors.js';
import {useEffect, useState} from "react";

function ColorSwatch({data, size} : {data: {color: string, name: string}, size: number}) {
    return (<div className={'swatches'} style={{backgroundColor: data.color, width: size + "px", height: size + "px", borderRadius: size / 2 + "px"}}>
        <p style={{color: 'white', fontSize: size / 8 + "px", fontFamily: "monospace"}}>{data.name}</p>
    </div>)
}

function App() {
    const [inputValue, setInputValue] = useState({color: "rgb(0, 0, 0)", name: "Black"});
    const [result, setResult] = useState({color: "rgb(0, 0, 0)", name: "Black"});
    const [ready, setReady] = useState<boolean>(false);

    useEffect(() => {
        if (ready) return
        init().then( () => {
            console.log("wasm initialized");
            setReady(true);
        })
    }, []);

    useEffect(() => {
        if (ready) {
            console.log("testing ")
            const rgb = closest('rgb(255, 0, 0)');
            console.log("testing rgb(255, 0, 0)", rgb )
            const hex = closest("#ff0000");
            console.log("testing #ff0000", hex );
            const rgba = closest('rgba(255, 0, 0, 147)');
            console.log("testing rgba(255, 0, 0, 147)", rgba )
            const hsl = closest('hsl(0, 100%, 50%)');
            console.log("testing hsl(0, 100%, 50%)", hsl )
        }
    }, [ready]);

    function toRGB(result: string) {
        const r = parseInt(result.substring(1, 3), 16)
        const g = parseInt(result.substring(3, 5), 16)
        const b = parseInt(result.substring(5, 7), 16)
        return {color:`rgb(${r}, ${g}, ${b})`, name: result}
    }

    function toHEX(rgb: string) {
        // Convert the rgb string (e.g., "rgb(255, 0, 0)") to a hex string
        const [r, g, b] = rgb.substring(4, rgb.length - 1).split(",").map(Number);

        // Ensure each component is two characters long with leading zeros
        const componentToHex = (c: number) => {
            const hex = c.toString(16);
            return hex.length === 1 ? '0' + hex : hex;
        };

        return `#${componentToHex(r)}${componentToHex(g)}${componentToHex(b)}`;
    }

    useEffect(() => {
        if (!ready) {
            return;
        }
        // Call the closest function with a color string
        const result: {color: string, name: string} = closest(inputValue.color);

        if (result?.name && result?.color) {
            setResult(result);
        } else {
            // Log the result to the console
            console.log(result);
        }

    }, [inputValue.color]);

    return (
        <>
            <div style={{display:"flex", gap: "2rem", padding: "2rem"}}>
                <ColorSwatch data={inputValue} size={160} /> <ColorSwatch data={result} size={160} />
            </div>
            <input
                type={"color"}
                value={inputValue.color}
                onChange={(e) => {
                        if (e.target.value) {
                            setInputValue({color:e.target.value, name: e.target.value})
                        }
                    }
                }
            />
            <p>☝️ choose a color</p>
        </>
    )
}

export default App
