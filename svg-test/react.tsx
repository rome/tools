type VNode = {  
    type: string;
    props: any;
    children: Array<VNode|string>
  };
  function createElement(  
    type: string,
    props: any,
    ...children: Array<VNode|string>
  ): VNode {
    return { type, props, children };
  }
  export default { createElement };  