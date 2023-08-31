import * as React from "react";
import { fluentProviderClassNames, useThemeClassName } from "@fluentui/react-components";
import { PortalCompatContextProvider } from "@fluentui/react-portal-compat-context";
import { applyFocusVisiblePolyfill } from "@fluentui/react-tabster";
import { RegisterPortalFn } from "@fluentui/react-portal-compat-context";
const CLASS_NAME_REGEX = new RegExp(`([^\\s]*${fluentProviderClassNames.root}\\w+)`);
export const PortalCompatProvider: React.FC<{
    children?: React.ReactNode;
}> = (props)=>{
    const { children } = props;
    const themeClassName = useThemeClassName();
    const cssVariablesClassName = React.useMemo<string | undefined>(()=>themeClassName.match(CLASS_NAME_REGEX)?.[1], [
        themeClassName
    ]);
    const registerPortalEl = React.useCallback<RegisterPortalFn>((element)=>{
        let disposeFocusVisiblePolyfill: () => void = ()=>undefined;
        if (cssVariablesClassName) {
            element.classList.add(cssVariablesClassName);
            if (element.ownerDocument.defaultView) {
                disposeFocusVisiblePolyfill = applyFocusVisiblePolyfill(element, element.ownerDocument.defaultView);
            }
        }
        return ()=>{
            if (cssVariablesClassName) {
                element.classList.remove(cssVariablesClassName);
            }
            disposeFocusVisiblePolyfill();
        };
    }, [
        cssVariablesClassName
    ]);
    if (process.env.NODE_ENV !== "production") {
        // This if statement technically breaks the rules of hooks, but ENV variables never change during app lifecycle
        // eslint-disable-next-line react-hooks/rules-of-hooks
        React.useEffect(()=>{
            if (themeClassName === "") {
                // eslint-disable-next-line no-console
                console.warn(/* #__DE-INDENT__ */ /** testando */ `
          PortalCompatProvider: "useThemeClassName()" hook returned an empty string
          =============================================
          Make sure that PortalCompatProvider is rendered inside FluentProvider as a child.
        `);
            }
        // eslint-disable-next-line react-hooks/exhaustive-deps
        }, []);
    }
    return null;
};
