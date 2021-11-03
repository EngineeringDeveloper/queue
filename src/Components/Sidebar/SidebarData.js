import React from 'react';
import ControlPointIcon from '@mui/icons-material/ControlPoint';
import FilterAltIcon from '@mui/icons-material/FilterAlt';
import TuneIcon from '@mui/icons-material/Tune';
import SettingsIcon from '@mui/icons-material/Settings';
import HelpCenterIcon from '@mui/icons-material/HelpCenter';
import EmojiEmotionsIcon from '@mui/icons-material/EmojiEmotions';

// https://mui.com/components/material-icons/
export const SidebarDataTop  = [
    {
        title:"Queue",
        icon: <EmojiEmotionsIcon />,
        link: "/home",
    },
    {
        title:"Add To Do",
        icon: <ControlPointIcon />,
        link: "/addToDOModal",
    },
    {
        title:"Filter",
        icon: <FilterAltIcon />,
        link: "/filterSidebar",
    },
    {
        title:"Tune",
        icon: <TuneIcon />,
        link: "/TuneModifications",
    },
]
export const SidebarDataBottom = [
    {
        title:"Settings",
        icon: <SettingsIcon />,
        link: "/Settings",
    },
    {
        title:"Help",
        icon: <HelpCenterIcon />,
        link: "/Help",
    },
]
