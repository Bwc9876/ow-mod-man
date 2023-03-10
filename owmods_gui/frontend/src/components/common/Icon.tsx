import { memo } from "react";
import { IconType } from "react-icons";

export interface IconProps {
    iconType: IconType;
    label?: string;
    iconClassName?: string;
}

// "Pure" icon component, use to prevent expensive rerenders
// TODO: Actually test this. Im on chromebook rn and react dev tools is blocked :(
const Icon = memo(
    (props: IconProps) => {
        return (
            <>
                {props.iconType({ className: props.iconClassName })}
                {props.label}
            </>
        );
    },
    (prev, next) => prev.label === next.label
);

export default Icon;
