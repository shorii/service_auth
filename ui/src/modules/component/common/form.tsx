import * as React from 'react';
import TextField from '@mui/material/TextField';
import Typography from '@mui/material/Typography';
import Paper from '@mui/material/Paper';
import Grid from '@mui/material/Grid';
import InputAdornment from '@mui/material/InputAdornment';
import AccountCircle from '@mui/icons-material/AccountCircle';
import Password from '@mui/icons-material/Password';
import Login from '@mui/icons-material/Login';
import Button from '@mui/material/Button';

export interface AuthFormValues {
    username: string;
    password: string;
}

const defaultAuthFormValues = {
    username: '',
    password: '',
};

export interface AuthFormProps {
    label: string;
    onClick: (values: AuthFormValues) => void;
    footer?: React.ReactNode;
}

export const AuthForm: React.FC<AuthFormProps> = (props) => {
    const { label, onClick, footer } = props;
    const [formValues, setFormValues] = React.useState<AuthFormValues>(defaultAuthFormValues);

    const handleClick = React.useCallback(async () => {
        await onClick(formValues);
    }, [onClick, formValues]);

    const handleChange = React.useCallback(
        (field: keyof AuthFormValues) => (e: React.ChangeEvent<HTMLInputElement>) => {
            setFormValues({
                ...formValues,
                [field]: e.target.value,
            });
        },
        [formValues],
    );

    return (
        <Grid container direction="column" justifyContent="center" alignItems="center">
            <Grid item xs={2}>
                <Paper elevation={3}>
                    <Grid container spacing={2} direction="column" style={{ padding: 16 }}>
                        <Grid item style={{ paddingTop: 0 }}>
                            <Typography variant="h5">{label}</Typography>
                        </Grid>
                        <Grid item>
                            <TextField
                                variant="standard"
                                label="username"
                                value={formValues.username}
                                onChange={handleChange('username')}
                                InputLabelProps={{
                                    shrink: true,
                                }}
                                InputProps={{
                                    startAdornment: (
                                        <InputAdornment position="start">
                                            <AccountCircle />
                                        </InputAdornment>
                                    ),
                                }}
                            />
                        </Grid>
                        <Grid item>
                            <TextField
                                variant="standard"
                                label="password"
                                value={formValues.password}
                                onChange={handleChange('password')}
                                InputLabelProps={{
                                    shrink: true,
                                }}
                                InputProps={{
                                    startAdornment: (
                                        <InputAdornment position="start">
                                            <Password />
                                        </InputAdornment>
                                    ),
                                }}
                            />
                        </Grid>
                        <Grid item>
                            <Button
                                variant="text"
                                startIcon={<Login />}
                                style={{ float: 'right' }}
                                onClick={handleClick}
                            >
                                {label}
                            </Button>
                        </Grid>
                    </Grid>
                </Paper>
                {footer && footer}
            </Grid>
        </Grid>
    );
};
