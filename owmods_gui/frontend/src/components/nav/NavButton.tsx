import { ReactNode } from "react";

export interface NavButtonProps {
    children: ReactNode;
    labelPlacement?: string;
    ariaLabel?: string;
    onClick?: () => void;
}

const NavButton = (props: NavButtonProps) => {
    return (
        <li>
            <a
                className="fix-icons"
                aria-label={props.ariaLabel}
                onClick={() => props.onClick?.()}
                data-tooltip={props.ariaLabel}
                data-placement={props.labelPlacement}
            >
                {props.children}
            </a>
        </li>
    );
};

export default NavButton;
