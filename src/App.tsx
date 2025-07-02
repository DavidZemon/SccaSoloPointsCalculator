import 'react-toastify/dist/ReactToastify.css';
import 'bootswatch/dist/slate/bootstrap.css';
import 'react-bootstrap-typeahead/css/Typeahead.css';
import { JSX, useCallback, useState } from 'react';
import { Col, Container, Row } from 'react-bootstrap';
import { toast, ToastContainer } from 'react-toastify';
import {
  ChampionshipType,
  SccaSoloPointsEngine,
} from 'scca_solo_points_engine/scca_solo_points_engine';
import { EventResults as EventResultsComponent } from './components/EventResults';
import { ChampionshipResults as ChampionshipResultsComponent } from './components/ChampionshipResults';
import { FileUploadBox } from './components/FileUploadBox';

export default function App(): JSX.Element {
  const [msrExportFile, setMsrExportFile] = useState<File | undefined>();
  const [eventResultsFile, setEventResultsFile] = useState<File | undefined>();
  const [championshipResultsFiles, setChampionshipResultsFiles] = useState<
    Partial<Record<keyof typeof ChampionshipType, File | undefined>>
  >({ Class: undefined, PAX: undefined, Novice: undefined, Ladies: undefined });
  const [pointsEngine, setPointsEngine] = useState<
    SccaSoloPointsEngine | undefined
  >();
  const [driversInError, setDriversInError] = useState<string[] | undefined>();
  const [championshipResults, setChampionshipResults] = useState<
    Partial<Record<keyof typeof ChampionshipType, string>> | undefined
  >();

  const processChampionships = useCallback(
    async (params?: {
      championshipType: keyof typeof ChampionshipType;
      newFile: File;
    }) => {
      if (params) {
        const { championshipType, newFile } = params;

        const mergedFiles = { ...championshipResultsFiles };
        mergedFiles[championshipType] = newFile;
        setChampionshipResultsFiles(mergedFiles);

        if (pointsEngine) {
          const resultsType = ChampionshipType[championshipType];
          const fileName = newFile.name;
          const newResults = pointsEngine.add_prior_championship_results(
            resultsType,
            new Uint8Array(await newFile.arrayBuffer()),
            fileName,
          );

          const newChampionshipResults: Partial<
            Record<keyof typeof ChampionshipType, string>
          > = {
            ...championshipResults,
          };
          newChampionshipResults[championshipType] = newResults;
          setChampionshipResults(newChampionshipResults);
        }
      } else {
        await Promise.all(
          Object.entries(championshipResultsFiles).map(
            async ([championshipType, newFile]) => {
              if (championshipType && newFile) {
                await processChampionships({
                  championshipType:
                    championshipType as keyof typeof ChampionshipType,
                  newFile,
                });
              }
            },
          ),
        );
      }
    },
    [championshipResultsFiles, championshipResults, pointsEngine],
  );

  return (
    <div>
      <ToastContainer pauseOnHover />

      <Container fluid>
        {/* Page header */}
        <Row>
          <Col>
            <h1>SCCA Solo Points Calculator v2</h1>
          </Col>
        </Row>

        {/* File upload boxes */}
        <Row>
          <Col>
            <FileUploadBox
              label={'MotorsportReg Export'}
              file={msrExportFile}
              accept={'.csv'}
              onFileSelect={async (f) => {
                setMsrExportFile(f);
                return true;
              }}
              fileSelectedMessage={(f) => (
                <p>
                  MSR export file <code>{f.name}</code> selected
                </p>
              )}
            />

            <FileUploadBox
              disabled={!msrExportFile}
              label={'Full Event Results (by class)'}
              file={eventResultsFile}
              accept={'.csv'}
              onFileSelect={async (f) => {
                try {
                  const rusty = new SccaSoloPointsEngine(
                    await msrExportFile!.text(),
                    await f.text(),
                  );
                  const driversInError =
                    rusty.js_drivers_in_error() as string[];
                  if (driversInError.length) {
                    console.error(`array=${JSON.stringify(driversInError)}`);
                    setEventResultsFile(f);
                    setDriversInError(driversInError);
                  } else {
                    setEventResultsFile(f);
                    setPointsEngine(rusty);
                    await processChampionships();
                  }

                  return true;
                } catch (e) {
                  if (
                    typeof e === 'string' &&
                    e.startsWith('Encountered an unexpected end of row')
                  ) {
                    toast.error(e);
                  } else {
                    console.error(e);
                    toast.error(
                      'File format does not match expected. Please export event results with raw times, grouped by class.',
                    );
                  }
                  return false;
                }
              }}
              fileSelectedMessage={(f) => (
                <>
                  <p key={'resultsSummary'}>
                    Showing results for <code>{f.name}</code> as new event
                    results.
                  </p>
                  {driversInError?.length ? (
                    <>
                      <p key={'errorIntro'}>
                        The following drivers appear to be in an error state:
                      </p>
                      <ul key={'errorList'}>
                        {driversInError.map((driver) => (
                          <li key={`driverInError-${driver}`}>{driver}</li>
                        ))}
                      </ul>
                      <p key={'demandRefresh'}>
                        Please fix the errors and refresh this page.
                      </p>
                      <p key={'fixInstructions'}>
                        To fix the errors, open TSAnnounce, search for each
                        class listed above, then re-run the export function from
                        TSAdmin
                      </p>
                    </>
                  ) : null}
                </>
              )}
            />
          </Col>

          <Col>
            {Object.keys(championshipResultsFiles).map(
              (champTypeStr, index) => {
                const championshipType =
                  champTypeStr as keyof typeof ChampionshipType;
                return (
                  <FileUploadBox
                    key={index}
                    label={`${championshipType} Championship Standings`}
                    accept={'.xls,.xlsx'}
                    file={championshipResultsFiles[championshipType]}
                    onFileSelect={async (f) => {
                      await processChampionships({
                        championshipType,
                        newFile: f,
                      });
                      return true;
                    }}
                    fileSelectedMessage={(f) =>
                      eventResultsFile ? (
                        <p>
                          Showing <strong>{championshipType}</strong>{' '}
                          championship standings based on <code>{f.name}</code>
                        </p>
                      ) : (
                        <p>
                          <code>{f.name}</code> set as{' '}
                          <strong>{championshipType}</strong> championship
                          standings. Please add event results to begin
                          processing.
                        </p>
                      )
                    }
                  />
                );
              },
            )}
          </Col>
        </Row>

        {pointsEngine && <EventResultsComponent pointsEngine={pointsEngine} />}

        <ChampionshipResultsComponent results={championshipResults} />
      </Container>
    </div>
  );
}
