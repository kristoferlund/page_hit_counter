import React, { useEffect, useState } from "react";

import { canisterId } from "../../declarations/counter/index";

export default function Counter() {
  const [imageLoaded, setImageLoaded] = useState(false);

  const counterSrc =
    import.meta.env.DEV ||
    window.location.hostname === "localhost" ||
    window.location.hostname === "127.0.0.1"
      ? `http://127.0.0.1:4943?canisterId=${canisterId}`
      : `https://${canisterId}.raw.icp0.io`;

  const loadingSrc = "/loading.svg";

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
