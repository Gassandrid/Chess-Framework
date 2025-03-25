"use client";

import { useState, useEffect } from "react";

// NOTE: I dont know if I will relaly put in the effort for it to wrok on mobile,
// but this is just standard practice for me at this point

export function useMobile() {
  const [isMobile, setIsMobile] = useState(false);

  useEffect(() => {
    const checkIfMobile = () => {
      setIsMobile(window.innerWidth < 768);
    };

    checkIfMobile();

    window.addEventListener("resize", checkIfMobile);

    return () => window.removeEventListener("resize", checkIfMobile);
  }, []);

  return isMobile;
}
