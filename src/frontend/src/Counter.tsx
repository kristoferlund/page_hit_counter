import React, { useEffect, useState } from "react";
import { canisterId, counter } from "../../declarations/counter/index";

export default function Counter() {
  const [imageLoaded, setImageLoaded] = useState(false);

  let counterSrc: string;
  let loadingSrc: string;

  if (
    import.meta.env.DEV ||
    window.location.hostname.includes("localhost") ||
    window.location.hostname === "127.0.0.1"
  ) {
    loadingSrc = `http://${canisterId}.localhost:4943`;
    counterSrc = `${loadingSrc}?track`;
  } else {
    loadingSrc = `https://${canisterId}.raw.icp0.io`;
    counterSrc = `${loadingSrc}?track`;
  }

  useEffect(() => {
    const image = new Image();
    image.src = counterSrc;
    image.onload = () => setImageLoaded(true);
  }, [counterSrc]);

  return imageLoaded ? (
    <img src={counterSrc} style={{ display: "inline" }} />
  ) : (
    <img src={loadingSrc} style={{ display: "inline" }} alt="Loading..." />
  );
}
