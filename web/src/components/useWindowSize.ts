import { useState, useEffect } from 'react';

const useWindowSize = () => {
  const [windowSize, setWindowSize] = useState({
    width: undefined,
    height: undefined,
  });
  useEffect(() => {
    const handleResize = () => {
      let obj: any = {
        width: window.innerWidth,
        height: window.innerHeight,
      }
      setWindowSize(obj);
    }
    
    window.addEventListener("resize", handleResize);
    
    handleResize();

    
    return () => window.removeEventListener("resize", handleResize);

  }, []);
  return windowSize;
}

export default useWindowSize